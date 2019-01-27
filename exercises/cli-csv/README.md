# Task
Write a Command-Line Interface (CLI) app for filtering CSV file containing coordinates and population of US cities. File can be downloaded from here (Links to an external site.)Links to an external site..

Download csv file:
`wget http://burntsushi.net/stuff/worldcitiespop.csv.gz`
 
 
 And unpack:
 `gunzip worldcitiespop.csv.gz`

Before starting this assignment read csv crate tutorial (Links to an external site.)Links to an external site. and structopt crate documentation (Links to an external site.)Links to an external site..

First filter out cities with population bigger than 100 000, but smaller than 1 000 000. Print results into stdout in the form of Tab Separated Values (TSV). (you'll get half of the points for completing this part)

Your next goal is to write an applications which will have the following flags:

--population-not-none will return only cities with known population

--population-none will return cities with unknown population

--population-gt <number> will return cities with population greater than given number

--population-lt <number> will return cities with population smaller than given number

--out <file_path> instead of printing results into standard output will write them into the file

Output of the application should be Tab Separated Values (TSV).
    
## Results
   
Input
![Alt text](images/1.png?raw=true "Input")
    
Output
![Alt text](images/2.png?raw=true "Output")
