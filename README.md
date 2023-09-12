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


### wardrobe 

Module for clothes and outfits.

##### get_user_clothes
##### update_user_clothes
##### get_new_outfit

### weather 

Module for weatehr forecasts.

##### 
