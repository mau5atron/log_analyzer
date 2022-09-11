// # The amount of times the URL was called.
// # mean (average) response times (connect time + service time)
// # median response times (connect time + service time)
// # mode of the response times (connect time + service time)
// # The "dyno" that responded the most.
// # The output should just be a simple hash structure.

// # {
// # 	"request_identifier":"POST /api/online/platforms/facebook_canvas/users/{resource_id}/add_ticket",
// # 	"called":3,
// # 	"response_time_mean":38.666666666666664,
// # 	"response_time_mode":33.0,
// # 	"response_time_median":33.0,
// # 	"dyno_mode":"web.12"
// # }

use std::cmp::Reverse;
use std::collections::{HashMap};
use std::fs::File;
use std::io::BufRead;
use std::vec; 
use regex::Regex; // Regex is an external crate (package)
use json::object; // json is external crate

fn get_split_offset(log_section: &str, pattern_find: &str) -> usize {
    let offset = log_section.find(pattern_find).unwrap() + 1;
    return offset;
}

fn get_parsed_string(log_section: &str) -> String {
    return log_section
            .to_string()
            .drain(get_split_offset(log_section, "=")..)
            .collect();
}

fn join_str_with_space(first: String, second: String) -> String {
    let mut new_str: String = first.clone();
    new_str.push_str(" ");
    new_str.push_str(&second);
   return new_str;
}

fn main(){
    let log_file: File = std::fs::File::open("./files/sample.log").expect("Unable to open log file");
    let resource_id_regex = Regex::new(r"\d+").unwrap();

    // actually needed to be a hashmap instead of hashset
    // used to store url endpoint + times url is called
    let mut urls_called_hm: HashMap<String, (i32, Vec<f32>, HashMap<String, i32>, f64)> = HashMap::new();

    // hashmap to store endpoint data
    // endpoint_data_hm -> 
    // response time is calculated by by adding (connect + service) in log
    // <String(endpoint), Vec<(i32(endpoint times called), Vec<f32>(response times list), Vec<String>(dyno list), f32(response_time_sum))>
    // let mut endpoint_data_hm: HashMap<String, Vec<(Vec<f32>, Vec<String>)>> = HashMap::new();
    // second item in hashmap holds tuples
    // let mut total_response_time: f32 = 0.0;
    // traversing log file O(n)

    if let file_buffer = std::io::BufReader::new(log_file){
        for line in file_buffer.lines(){
            if let Ok(log_line) = line {
                let splitted: Vec<&str> = log_line.split(" ").collect();
                // splitted[7] -> dyno, 
                // splitted[8] -> connect_time,
                // splitted[9] -> service_time

                
                // CONNECT TIME
                let connect_time_range = resource_id_regex.find(splitted[8]);
                let mut connect_time: String = String::from("");
                match connect_time_range {
                    // Some(int_match) => println!("service time int between {} and {}", int_match.start(), int_match.end()),
                    Some(int_match) => 
                    connect_time = splitted[8].to_string().drain(int_match.start()..int_match.end()).collect::<String>(),
                    None => continue
                }
                
                // print!("connect time: {:?}, ", connect_time.parse::<f32>().unwrap());
                
                let mut example_vec = vec![1, 5, 6];
                example_vec.sort_by_key(|key| Reverse(*key));
                
                // SERVICE TIME
                let service_time_range = resource_id_regex.find(splitted[9]);
                let mut service_time: String = String::from("");
                match service_time_range {
                    // Some(int_match) => println!("service time int between {} and {}", int_match.start(), int_match.end()),
                    Some(int_match) => 
                        service_time = splitted[9].to_string().drain(int_match.start()..int_match.end()).collect::<String>(),
                    None => continue
                }

                // println!("Service time: {:?}\n", service_time.parse::<f32>().unwrap());
            
                

                // REQUEST METHOD + ENDPOINT
                let mut parsed_req_method_with_endpoint = get_parsed_string(splitted[3]);
                let parsed_endpoint = get_parsed_string(splitted[4]);

                parsed_req_method_with_endpoint = join_str_with_space(parsed_req_method_with_endpoint, parsed_endpoint);
                
                let resource_id_range = resource_id_regex.find(parsed_req_method_with_endpoint.as_str());

                match resource_id_range {
                    // replacing the integer values to avoid entering unique endpoints 
                    // due to resource_id being different for each log line
                    Some(int_match) => parsed_req_method_with_endpoint.replace_range(int_match.start()..int_match.end(), "{resource_id}"),
                    None => continue 
                }

                // println!("REQUEST METHOD---------------{}", parsed_req_method_with_endpoint);

                // DYNO
                let mut parsed_dyno = splitted[7].to_string();
                let dyno_parse_range = parsed_dyno.find("=").unwrap()+1;
                parsed_dyno = parsed_dyno.drain(dyno_parse_range..).collect();

                // println!("DYNOOOOO: {}", parsed_dyno);

                // HashMap lookup O(1)
                if urls_called_hm.contains_key(&parsed_req_method_with_endpoint){
                    // if key exists, update counter for calls made in place

                    // <String(endpoint),

                    // (
                    // Vec<(i32(endpoint times called), 
                    // Vec<f32>(response times list), 
                    // HashMap<String(dyno), i32(times called)>
                    // f32(response_time sum)
                    // )>
                    let response_time = connect_time.parse::<f64>().unwrap() + service_time.parse::<f64>().unwrap();
                    println!("Response time = {}", response_time);
                    let url_data_counter: &mut (i32, Vec<f32>, HashMap<String, i32>, f64) = urls_called_hm
                                                                                .entry(parsed_req_method_with_endpoint)
                                                                                .or_insert((0, vec![], HashMap::new(), 0.0));
                    url_data_counter.0 += 1; // adding url call count in tuple
                    // update other data here
                    url_data_counter.3 += response_time as f64; // summing response time to get average

                    // if url_data_counter.2.contains_key(&parsed_dyno){
                        let dyno_counter = url_data_counter.2.entry(parsed_dyno).or_insert(0);
                        *dyno_counter += 1;
                    // } else {
                    //     url_data_counter.2.insert(parsed_dyno, 1);
                    // }
                } else {
                    // else insert/instantiate key with val = 1
                    let mut dyno_hash_map = HashMap::new();
                    dyno_hash_map.insert(parsed_dyno, 1);
                    urls_called_hm.insert(parsed_req_method_with_endpoint, (1, vec![], dyno_hash_map, 0.0));
                }

                // Todo:
                // add mean, median, mode
                // mean is calculated by adding all numbers, then dividing by data set size

                // median is calculated by finding the middle of the sorted data set, if two values are in the middle
                // then find the average of the two
                
                // mode is the number in a dataset that occurs most frequently

                // Need to find dyno (server) that responded the most
                
                // To solve the rest of the problem, will need to create another HashMap
                // that collects multiple lists of data

                // Example:

                /*
                    let endpoint_data: HashMap<String, Vec<Vec<f32>>>...
                    each corresponding value to the endpoint string key in hashmap
                    should look like this:

                    vec![
                        vec![], // hold response time values to calculate mean, median, mode
                        vec![], // hold list of dynos to figure out which responded most
                                    // will strip off string data so that web.12 becomes 12.00
                                    // and format back to initial string after calculations are done
                    ];
                */
            }
        }
    }

    let mut i: i32 = 0;
    for (url, url_data) in urls_called_hm.iter(){
        let times_url_called: String = url_data.0.to_string();
        let average_response_time: f64 = url_data.3/(url_data.0 as f64);
        println!("{} divided by {}", url_data.3, (url_data.0 as f64));
        let most_called_dyno = url_data.2.iter().max_by_key(|entry| entry.1).unwrap().0;
        let obj = object! {
            request_identifier: url.to_string(),
            called: times_url_called,
            response_time_mean: average_response_time,
            response_time_mode: "fill_out",
            response_time_median: "fill_out",
            dyno_mode: most_called_dyno.to_string()
        };
        
        if i < (urls_called_hm.len() as i32)-1{
            print!("{:#},\n", obj);
        } else {
            print!("{:#}\n", obj);
        }

        i+=1;
    }

    /*
        Example output:

        {
            "request_identifier": "GET /api/users/{resource_id}/get_friends_progress",
            "called": "1117"
        }
        ........
    */
}