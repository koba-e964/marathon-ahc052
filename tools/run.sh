set -eu
ID=0000
<in/${ID}.txt ../a >out/${ID}.txt
cargo run -r --bin vis in/${ID}.txt out/${ID}.txt
