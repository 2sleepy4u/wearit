# Untitled 

## API entry points Standard

[module]/[function]

Module: rapresent the context  
Function: rapresent the specific function which name also follow a standard

action_context_something

## Modules

### /

Root module: it manages default behaviour.  

##### Ping
Pong!

### auth

Authentication module

##### get_new_session

Check if user is valid, if so returns SSID token (session token).  

Method: POST  
Body: {
    email: String,
    password: String
    }  


### api

Api module 

##### 
