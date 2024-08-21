## Rick's Inventory application

### First Implementation analysis
What is wrong with this code? Why the search method continuously returned nothing?
Looking at the code, we can easily enumerate the following points:
1. We are using a lots of case sensitive string comparisons in a strict way. Meaning a simple difference in case sensitiveness between Erin's guitar features and the corresponding guitar in the database will lead to an unsuccessful search.
2. Strings can be replaced with constant or some other object
3. What if more than one guitars meet the searching criteria? In the current implementation, the search function stops on the first guitar that meets the search criteria and return it. This doesn't give the choice to the user on the matching guitars.
4. Inventory and Guitar classes are dependent upon each other. Let's say we want to add another field to the Guitar class, that need to be accounted for as search criteria. Then, we will be required to update the search method in the Inventory class as well to account for it. 
5. Guitar fields like serial_number and price are not used as search criteria so, giving a whole guitar object to the search method looks like giving too much information.

### Great software definition
The definition of great software varies depending on the profile giving the definition.

#### Customer-friendly programmer:
> Great software always does what the customer wants it to. So even if the customers think of new ways to use the software, it doen't break or give them unexpected results.

#### Object-oriented programmer definition:
> Great software is code that is object-oriented. So there's not a bunch of duplicate code, and each object pretty much controls its own behavior. It's also easy to extend because your design is really solid and flexible.

#### Design-guru programmer definition:
> Great software is when you use tried-and-true design patterns and principles. You've kept your object loosely coupled, and your code open for extension but closed for modification. That also helps make the code more reusable, so you don't have to rework everything to use parts of your application over and over again.

### What do you think "Great Software" means?
Great software is not just one thing. It must:
1. Satisfy the customer. The software must do what the customer wants it to do.
2. Is well-designed, well-coded, and easy to maintain, reuse, and extend.

#### One can design a great software following these three easy steps:
1. Make sure your software does what the customer wants it to do.
2. Apply basic OO principles to add flexibility.
3. Strive for maintainable, reusable design.