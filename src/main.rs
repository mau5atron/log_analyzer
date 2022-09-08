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
use regex::Regex; // Regex is an external crate (package)
use json::object; // json is external crate

fn get_split_offset(log_section: &str, pattern_find: &str) -> usize {
    let offset = log_section.find(pattern_find).unwrap() + 1;
    return offset;
}

fn main(){
    let log_file: File = std::fs::File::open("./files/sample.log").expect("Unable to open log file");
    let resource_id_regex = Regex::new(r"\d+").unwrap();

    // actually needed to be a hashmap instead of hashset
    // used to store url endpoint + times url is called
    let mut urls_called_hm: HashMap<String, i32> = HashMap::new();

    // traversing log file O(n)
    if let file_buffer = std::io::BufReader::new(log_file){
        for line in file_buffer.lines(){
            if let Ok(log_line) = line {
                let splitted: Vec<&str> = log_line.split(" ").collect();
                let mut parsed_req_method_with_endpoint: String = splitted[3]
                                                                  .to_string()
                                                                  .drain(get_split_offset(splitted[3], "=")..)
                                                                  .collect();
                let parsed_endpoint = splitted[4]
                                              .to_string()
                                              .drain(get_split_offset(splitted[4], "=")..)
                                              .collect::<String>();

                // joining request method with endpoint called
                parsed_req_method_with_endpoint.push_str(" ");
                parsed_req_method_with_endpoint.push_str(&parsed_endpoint);
                
                let resource_id_range = resource_id_regex.find(parsed_req_method_with_endpoint.as_str());

                match resource_id_range {
                    // replacing the integer values to avoid entering unique endpoints 
                    // due to resource_id being different for each log line
                    Some(int_match) => parsed_req_method_with_endpoint.replace_range(int_match.start()..int_match.end(), "{resource_id}"),
                    None => continue 
                }

                // HashMap lookup O(1)
                if urls_called_hm.contains_key(&parsed_req_method_with_endpoint){
                    // if key exists, update counter for calls made in place
                    let counter = urls_called_hm.entry(parsed_req_method_with_endpoint).or_insert(0);
                    *counter+=1;
                } else {
                    // else insert/instantiate key with val = 1
                    urls_called_hm.insert(parsed_req_method_with_endpoint, 1);
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
    for (url, called) in urls_called_hm.iter(){
        let obj = object! {
            request_identifier: url.to_string(),
            called: called.to_string()
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