# test

 

cargo watch -x run



### Run local DB

docker run -p 9000:9000  \
      -p 8812:8812 \
      -p 9009:9009 \
      questdb/questdb