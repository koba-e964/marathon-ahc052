set -eu
for i in {0..0}; do
    ID=000${i}
    <in/${ID}.txt ../a >out/${ID}.txt
    cargo run -r --bin vis in/${ID}.txt out/${ID}.txt
done
