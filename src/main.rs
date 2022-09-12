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

use std::collections::{HashMap};
use std::fs::File;
use std::io::BufRead;
use std::vec; 
use regex::{Regex, Match}; // Regex is an external crate (package)
use json::{object, JsonValue}; // json is external crate

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
    let numbers_regex = Regex::new(r"\d+").unwrap();

    // actually needed to be a hashmap instead of hashset
    // urls_called_hm holds a tuple with relevant data to calculate mean, median, mode, most called dyno etc
    let mut urls_called_hm: HashMap<String, (i32, Vec<i32>, HashMap<String, i32>, f64, HashMap<i32, i32>)> = HashMap::new();
    // second item in hashmap holds tuples

    // traversing log file O(n)
    if let file_buffer = std::io::BufReader::new(log_file){
        for line in file_buffer.lines(){
            if let Ok(log_line) = line {
                let splitted: Vec<&str> = log_line.split(" ").collect();
                // splitted[7] -> dyno, 
                // splitted[8] -> connect_time,
                // splitted[9] -> service_time
                
                // CONNECT TIME
                let connect_time_range: Option<Match> = numbers_regex.find(splitted[8]);
                let mut connect_time: String = String::from("");
                match connect_time_range {
                    // Some(int_match) => println!("service time int between {} and {}", int_match.start(), int_match.end()),
                    Some(int_match) => 
                    connect_time = splitted[8].to_string().drain(int_match.start()..int_match.end()).collect::<String>(),
                    None => continue
                }
                
                // SERVICE TIME
                let service_time_range: Option<Match> = numbers_regex.find(splitted[9]);
                let mut service_time: String = String::from("");
                match service_time_range {
                    // Some(int_match) => println!("service time int between {} and {}", int_match.start(), int_match.end()),
                    Some(int_match) => 
                        service_time = splitted[9].to_string().drain(int_match.start()..int_match.end()).collect::<String>(),
                    None => continue
                }
            
                // REQUEST METHOD + ENDPOINT -> "GET /api/users/{resource_id}/get_friends_progress"
                let mut parsed_req_method_with_endpoint: String = get_parsed_string(splitted[3]);
                let parsed_endpoint = get_parsed_string(splitted[4]);
                parsed_req_method_with_endpoint = join_str_with_space(parsed_req_method_with_endpoint, parsed_endpoint);
                let resource_id_range: Option<Match> = numbers_regex.find(parsed_req_method_with_endpoint.as_str());
                match resource_id_range {
                    // replacing the integer values to avoid entering unique endpoints 
                    // due to resource_id being different for each log line
                    Some(int_match) => parsed_req_method_with_endpoint.replace_range(int_match.start()..int_match.end(), "{resource_id}"),
                    None => continue 
                }

                // DYNO
                let mut parsed_dyno: String = splitted[7].to_string();
                let dyno_parse_range: usize = parsed_dyno.find("=").unwrap()+1;
                parsed_dyno = parsed_dyno.drain(dyno_parse_range..).collect();

                let response_time: f64 = connect_time.parse::<f64>().unwrap() + service_time.parse::<f64>().unwrap();
                if urls_called_hm.contains_key(&parsed_req_method_with_endpoint){
                    // if key exists, update counter for calls made in place

                    // API called hashmap structure
                    // <String (req method and endpoint), ()>
                    //                                     ^ tuple with the rest of our data

                    // String endpoint tuple values:
                    /*
                        (
                            i32, 0 (times endpoint called)
                            Vec<i32>, 1 (response times lists for specific endpoint)
                            HashMap<String, i32>, 2 (times dynos were called)
                            f64, 3 (aggregated response times)
                            HashMap<i32, i32>, 4 (response time hashmap to get mode (most occurring response time))
                        )
                    */
                        // (
                            // i32, 0 (times endpoint called)
                            // Vec<i32>, 1 (response times lists for specific endpoint)
                            // HashMap<String, i32>, 2 (times dynos were called)
                            // f64, 3 (aggregate response times)
                            // HashMap<i32, i32>, 4 (response time hashmap to get mode (most occurring response time))
                        // )

                    let url_data_counter: &mut (i32, Vec<i32>, HashMap<String, i32>, f64, HashMap<i32, i32>) = 
                        urls_called_hm
                        .entry(parsed_req_method_with_endpoint)
                        .or_insert((0, vec![], HashMap::new(), 0.0, HashMap::new()));
                    url_data_counter.0 += 1; // URL call count
                    url_data_counter.1.push(response_time as i32); // pushing response time to list
                    if url_data_counter.2.contains_key(&parsed_dyno){
                        let dyno_counter: &mut i32 = url_data_counter.2.entry(parsed_dyno).or_insert(0);
                        *dyno_counter += 1;
                    } else {
                        url_data_counter.2.insert(parsed_dyno, 1);
                    }

                    url_data_counter.3 += response_time as f64; // summed response time

                    let response_time_counter: &mut i32 = url_data_counter.4.entry(response_time as i32).or_insert(0);
                    *response_time_counter += 1;
                } else {
                    // else insert/instantiate key with val = 1
                    let mut dyno_hash_map: HashMap<String, i32> = HashMap::new();
                    dyno_hash_map.insert(parsed_dyno, 1); // init dyno hashmap to 1 to init on urls called data hashmap
                    let mut most_frequent_response_time_hs: HashMap<i32, i32> = HashMap::new();
                    most_frequent_response_time_hs.insert(response_time as i32, 1);
                    // response time being 0 for single call dynos was due to init not having response time set
                    urls_called_hm.insert(parsed_req_method_with_endpoint, (1, vec![response_time as i32], dyno_hash_map, response_time as f64, most_frequent_response_time_hs));
                }

                // Todo: DONE
                // add mean, median, mode
                // mean is calculated by adding all numbers, then dividing by data set size

                // median is calculated by finding the middle of the sorted data set, if two values are in the middle
                // then find the average of the two
                
                // mode is the number in a dataset that occurs most frequently // another hashmap
            }
        }
    }

    let mut i: i32 = 0;
    let urls_hash_length = urls_called_hm.len();
    for (url, url_data) in urls_called_hm.iter_mut(){
        let times_url_called = url_data.0; 
        let average_response_time: f64 = url_data.3/(url_data.0 as f64); // summed response time / times url called
        let most_called_dyno: &String = url_data.2
                                        .iter()
                                        .max_by_key(|entry| entry.1)
                                        .unwrap().0; // sort dyno hashmap by calls, get most called dyno
        
        let mut median = 0.0;
        url_data.1.sort();
        if url_data.1.len() % 2 == 0 {
            let mid = url_data.1.len() as f64 / 2.00;
            median = (url_data.1[mid as usize] + url_data.1[mid as usize - 1]) as f64/2.0;
        } else {
            let mid = url_data.1.len() as f64/2.0;
            median = url_data.1[mid as usize] as f64;
        }

        // MODE
        let most_occurring_response_time: &i32 = url_data.4
                                                            .iter()
                                                            .max_by_key(|entry| entry.1)
                                                            .unwrap().0;
        let obj: JsonValue = object!{
            request_identifier: url.to_string(),
            called: times_url_called,
            response_time_mean: average_response_time,
            response_time_mode: most_occurring_response_time.to_owned(),
            response_time_median: median,
            dyno_mode: most_called_dyno.to_string()
        };
        
        // in web version, render objects in browser as json
        if i < (urls_hash_length as i32)-1{
            println!("{:#},", obj); // before last
        } else {
            println!("{:#}", obj); // last
        }

        i+=1;
    }

    /*
        Example output:

        {
            "request_identifier": "GET /api/users/{resource_id}/get_friends_progress",
            "called": "1117",
            "response_time_mean": 111.89704565801253,
            "response_time_mode": 35,
            "response_time_median": 51,
            "dyno_mode": "web.5"
        }
        ........
    */
}