# Log analyzer
#
# The objective is to parse a log file and do some analysis on it.
#
# Requirements & output
#
# The log file contains all requests to a server within a specific timeframe. Given the following method/url definitions:
#
# GET /api/users/{user_id}/count_pending_messages
# GET /api/users/{user_id}/get_messages
# GET /api/users/{user_id}/get_friends_progress
# GET /api/users/{user_id}/get_friends_score
# POST /api/users/{user_id}
# GET /api/users/{user_id}
# POST /api/online/platforms/facebook_canvas/users/{resource_id}/add_ticket
# Where user_id is the user id of the users calling the backend.
#
# The script/program should output a small analysis of the sample log containing at the following:
#
# For each of the URLs above:
#
# The amount of times the URL was called.
# mean (average) response times (connect time + service time)
# median response times (connect time + service time)
# mode of the response times (connect time + service time)
# The "dyno" that responded the most.
# The output should just be a simple hash structure.
#
# For example:
#
# {
# 	"request_identifier":"POST /api/online/platforms/facebook_canvas/users/{resource_id}/add_ticket",
# 	"called":3,
# 	"response_time_mean":38.666666666666664,
# 	"response_time_mode":33.0,
# 	"response_time_median":33.0,
# 	"dyno_mode":"web.12"
# }
#
# Log format
#
# The logformat is defined as:
#
# {timestamp} {source}[{process}]: at={log_level} method={http_method} path={http_path} host={http_host} fwd={client_ip} dyno={responding_dyno} connect={connection_time}ms service={processing_time}ms status={http_status} bytes={bytes_sent}
# Example:
#
# 2014-01-09T06:16:53.916977+00:00 heroku[router]: at=info method=GET path=/api/users/1538823671/count_pending_messages host=mygame.heroku.com fwd="208.54.86.162" dyno=web.11 connect=7ms service=9ms status=200 bytes=33

# This part is starter in Ruby
# require 'open-uri'

# small_file_url = 'https://gist.githubusercontent.com/bss/6dbc7d4d6d2860c7ecded3d21098076a/raw/244045d24337e342e35b85ec1924bca8425fce2e/sample.small.log'
# large_file_url = 'https://gist.githubusercontent.com/bss/1d7b8024451dd45feb5f17e148dacee5/raw/b02adc43edb43a44b6c9c9c34626243fd8171d4e/sample.log'

# open(small_file_url) do |f|
  # puts f.readlines.first(10)
# end
# */