cargo run -- --no_summary --algo=gd | sed -e "s/\[//g" -e "s/\]//g" > gd.csv
cargo run -- --no_summary --algo=rmsprop | sed -e "s/\[//g" -e "s/\]//g" > rmsprop.csv
cargo run -- --no_summary --algo=rmsprop_momentum | sed -e "s/\[//g" -e "s/\]//g" > rmsprop_momentum.csv