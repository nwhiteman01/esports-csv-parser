Nicholas Whiteman
Esports CSV Parser

About:
Esports CSV Parser is a program that takes in the mega data from Oracle’s Elixir CSV League of Legends file and parses that data. 
It specifically looks for LCK players and calculates the total amount of fantasy points for that week. It will then store that
into a .csv file that keeps track of that information for the next week. The scoring system was designed by me and is posted at the bottom of this README.
The fantasy "coaches" have five players (one per role) and a LCK team they can add to their roster. Coaches can edit their roster as well as
see, calculate scores of players and teams!

How to Build:
If you just want to use the data.csv I have provided, all you need to do is type "cargo run" like a normal Rust file. 
However, if you plan to update this and place data for future weeks, you will need to edit your CSV file. To do so, 
download the CSV from Oracle Elixir, under downloads. You will then need to manually remove the dates previous to the week 
you are calculating for. I personally recommend you only use this program for data calculating on Sunday Afternoon each week, and
just do the full previous LCK week at once. Once you have only that week's worth of games (do not worry about filter by region, the
program already does so), rename it to "data.csv" and replace the file in the ESPORT-CSV-Parser folder where the current "data.csv" is.
It should then work! 

The actual program is straight forward, your menus will determine what you plan on doing. I personally recommend checking the Pro Stats
menu first to see all the parsing, then you could add your own coach and so on in the other menus.

I put in warnings for calulcations, please only calculate once and save once as it will continue adding points every time you calculate. 
I will discuss future options in my "Plans" section. 

rustdoc: cargo doc --open 

Process:
This code is based off of my own personal project named "fantasyLCK". These both accomplish the same thing, but "fantasyLCK" is in C++. 
This Esports CSV Parser is also more advanced as it uses the actual CSV rather than "fantasyLCK" where I had to convert things to TXT
files. My scope on this project was far too big for the time I had to work on it. I wanted to make it so the program would update
on its own and parse through the CSV, only taking the last week worth of data, so I wouldn't have to edit a CSV file manually every time. 
Unfortunately, dealing with time is a lot more complicated than I previously had thought so I had to scrap the idea last minute, in order
to get a working program done on time. The actual program runs very well and is almost a near copy of my previous C++ attempt, which does
making me very happy. Serde is a wonderful framework that has made my life tremendously easy, allowing me to deserialize and serialize 
without hassle. It also blended well with the CSV crate which was perfect for my needs here. I probably should've added some more
testing in my program, but I did a lot of testing by hand which was very inefficient, I will work on that. 

Plans:
I plan to continue updating this program. My big stretch goal is to make it more self sufficent so I don't have to import a CSV by hand. 
It will either be through reqwest, where I download the CSV automatically from Oracle's Elixir, or I will try and get mwbot-rs/Wikibase RS
to work and grab data straight from the Leaguepedia API. Another thing I will add is a way to block the user from running calculation code
twice. This won't be necessary when I get the code to auto update data, but for now I need it to prevent mistakes. Hopefully I can
turn this into something more and continue making it more accessible for the average user.  

Special thanks to these source of information that have currently helped me with creating this code. 
Oracle's Elixir: https://oracleselixir.com/ - This is literally the whole base of this project!
Serde: https://serde.rs/ - Seralizing & Deserializing
Crate csv: https://docs.rs/csv/latest/csv/ - Working with csv's, great tutorial!
Microsoft Copilot: https://copilot.microsoft.com/ - Helping me when I got stuck in niche scenarios and for helping me learn more details about Rust in general
and of course...
CS523 Rust Programming 001!! - For starting me on the Rust journey!

Designed and Tested by Me (last updated: June 6th, 2024)
Scoring System:
   • Top & Jungle
      • Kills: 0.4 pts
      • Assists: 0.2 pts
      • Deaths: -0.15 pts
      • CS: 0.002 pts
   • Mid & Bot
      • Kills: 0.4 pts
      • Assists: 0.15 pts
      • Deaths: -0.2 pts
      • CS: 0.0015 pts
   • Support
      • Kills: 0.3 pts
      • Assists: 0.25 pts
      • Deaths: -0.1 pts
      • CS: 0.003 pts
   • Team
      • Game Win: 2.5 pts