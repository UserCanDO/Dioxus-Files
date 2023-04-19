# **Findings**
Dioxus is cross platform development framework, which is meant to be run on the Web, Mobile Devices and PC. It is yet to receive a stable realise, that means components are being changed constantly or moved arround therefore affecting the whole architecture.
with that here are my findings:

## Pros:

### Cross Platform
As stated, this frameworks aims to be completely cross-platform, targeting all current platforms whether Mobile(IOS and Android), Desktop(Linux, Windows, Mac) and Web. all with little to no changes.

### Architecture
It has a straight forward component based architecture like React, where everything is a component, with very easy and highly customizable syntax. it is a lot like Yew, but unlike Yew that tries to be React as much as posible, Dioxus goes a  slightly different direction which fills more organic for a Rust developer, instaed of using tags, you use structs which makes more familiar to a Rust Developer.

### Customizability
It is an under statement when I say Dioxus is very customizable,  as every component is created by the devloper with no blueprint or strict rules to follow, just create a function that return an  Element and you are done, the sty is the limit to what you can do.
adding css files for styling is straight forward and simple whether it is internal or in a serer. and one chocking feature is you can add javascript, GLSL and JSON script to your components.

## Cons:

### Poor Documentation
As stated earlier, Dioxus is still in development stage and yet to receive a stable release, they have come along way as the overal core architecture has taken shape. but there are still things to iron out and a lot of things that can be changed anytime.
It is for this reason I beleive the Documentation feels and is unfinished, as major areas such as Images are left out, only componets they know will not change are documented.
Most examples provided are for Desktop development which means example for Web are few in between. not to metion Moblie Example are  almost none existant.

### Community
This was a great suprise, because Rust is a very much loved language, and any library or framework written in Rust normally have large number of developers supporting and providing guides and Tutorials.
But as It stands, as of now, there are little to no Tutorials out there. couple this with poor documentation and very few exaples to pull from. you are left alone trying to figure out everything yourself as you can not turn to someone on the internet who might have faced similar issue.
For Example, I want to do something as simple as to detect when the user clickes anywhere on the screen, so I can close any context menu that is open. in javascript is as simple  as:
``` 
    document.body.addEventListener("click", (){
        .....
    });
```
I can't find the equivalent anywhere, enen after checking the documentation and examples which as stated before is mainly for desktop and I am developing for web. I even decided to ask ChatGPT 2, but the code it generated was unrecognizable. 

```
    {% use "base.html" %}

    {% block content %}
    <div class="container">
    <h1>My Dioxus Page</h1>
    <button id="my-button" class="btn btn-primary">Click me</button>
    </div>

    {% script %}
    document.getElementById("my-button").addEventListener("click", function() {
    alert("Button clicked!");
    });
    {% endscript %}
    {% endblock %}
```

I quess this is how the framework used to look like when it initially started development. as i was using ChatGPT 2.

### Compiling and Hot Reload
This may be caused by my PC, but as someone who uses React, and other frontend developemt framework, the compile time for Dioxus is less desirable as it halts your production untils it compiles fully. it is aproximately 3 times slower than React with webpack not to even mention Vite.
The Hot reload to is a bit better but considering it reloads everytime you save single file. you have to wait a staggering 5 to 8 seconds before it loads and refreshes the screen every single time you accidently save a single line of code.

## Summary
Dioxus is a great framework and I love it more than React for what it is trying to achieve and simplistic it is, but poor documentation and lack of resources to work with makes this a  bit hard to get into and so many moving parts needed for development are not accounted for. but it has alot of potential to be the next top development framework once all the  kinks have been ironed out.
