cargo run -- --no_summary --algo=gd | sed -e "s/\[//g" -e "s/\]//g" > output/gd.csv
cargo run -- --no_summary --algo=rmsprop | sed -e "s/\[//g" -e "s/\]//g" > output/rmsprop.csv
cargo run -- --no_summary --algo=rmsprop_momentum | sed -e "s/\[//g" -e "s/\]//g" > output/rmsprop_momentum.csv
