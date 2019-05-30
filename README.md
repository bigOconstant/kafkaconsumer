# Rusty Oculus Kafka consumer

Rusty Oculus Kafka consumer is a Rust Crate intended to do one thing. Consume messages. 

## Installation

add the following line to your dependencies in your cargo.toml
```
kafkaconsumer = { git = "ssh://git@github.ibm.com/Caleb-Andrew-McCarthy/rustkafkaconsumer", branch = "master" }

```


## Usage

```
extern crate kafkaconsumer;
pub fn handleMessage(input:&str)-> String{
   // input is kafka message. do stuff with it
  ""._to_string()
}

kafkaconsumer::consume_kafka_messages("broker".to_string(),
                                  "group id".to_string(),
                                  "topic".to_string(),
                                  "sasluser".to_string(),
                                  "saslpassword".to_string(),
                                  handleMessage)
```

Thats it. kafkaconsumer takes care of loging and everything else. You just write a function to handle the message and pass it to the consumer.
