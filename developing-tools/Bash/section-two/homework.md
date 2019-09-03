# Instructions

Please fill out the text file and submit it via Git like so

```
1A. &
1B. |
1C. >
1D. ;
```

Fill out the script `animal-names` and `animal-color-generator` with their
solutions. 


## Questions

1. The following command is missing some things. What are the answers to A, B, C, and D that make
the following command print whether or not you are connected to the internet. The output should be
as follows. 

`ping -c 1 google.com A /dev/null 2>&1 B [[ $? = 0 ]] C echo "You are connected to the internet" D echo "You are not connected to the internet"`

```
196$ (running command when connected)
You are connected to the internet

196$ (running command when not connected to the internet)
You are not connected to the internet
```

### Animals Script
Use `curl` and `jq` in one command to print out the names of the animals in [this
file](https://raw.githubusercontent.com/LearnWebCode/json-example/master/animals-1.json).
[Hint](https://stedolan.github.io/jq/manual/)

Output should look like 
```
Meowsy
Barky
Purrpaws
``` 

and not like 

```
"Meowsy"
"Barky"
"Purrpaws"
```

### Color animal generator.
Write a bash script that asks a user for a color and their favorite animal and output them
together. For example,

```
196$ bash color-animal-generator
What is your favorite color? 
What is your favorite animal? 
Red Koala
```

