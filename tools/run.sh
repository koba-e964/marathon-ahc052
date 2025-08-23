set -eu
for i in {0..4}; do
    ID=000${i}
    <in/${ID}.txt ../a >out/${ID}.txt
    cargo run -r --bin vis in/${ID}.txt out/${ID}.txt
done
