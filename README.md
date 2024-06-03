# albatros_frontend

# Introduction

## Environment setup

### Direnv

To manage dependencies the nix based [direnv](https://direnv.net/) is encouraged. Otherwise dependencies must be managed imperatively and no guide is provided
for installing the dependencies on any system.

Follow the instructions on how to setup direnv for your system.

### Just

To run any tasks [just](https://github.com/casey/just) is used.

# Appendix

## A note on components

Coming from OOP UI frameworks like react, angular etc. the urge to divide everything into components is certain.
The Elm Guide (which iced-rs is heaviley inspired by) states the following concerning components:

>Folks coming from React expect everything to be components. Actively trying to make components is a recipe for disaster in Elm. The root issue is that components are objects:
>
>    components = local state + methods
>
>    local state + methods = objects
>
>It would be odd to start using Elm and wonder "how do I structure my application with objects?" There are no objects in Elm! Folks in the community would recommend using custom types and functions instead.
>
>Thinking in terms of components encourages you to create modules based on the visual design of your application. “There is a sidebar, so I need a Sidebar module.” It would be way easier to just make a viewSidebar function and pass it whatever arguments it needs. It probably does not even have any state. Maybe one or two fields? Just put it in the Model you already have. If it really is worth splitting out into its own module, you will know because you will have a custom type with a bunch of relevant helper functions!
>
>Point is, writing a viewSidebar function does not mean you need to create a corresponding update and Model to go with it. Resist this instinct. Just write the helper functions you need.
