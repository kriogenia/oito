# Exceptions

My next step was starting to give functionality to the most basic components, like the Stack.
I decided to make the most barebone Stack that I could, just `push` and `pop`.
The thing is that those operations can fail, pushing into a full stack will give the famous stack overflow exception
and using pop into an empty stack will give an underflow exception.

## Error or Panic

Now, how should I handle this situation? Just let Rust send the panic that would occur trying to access a wrong index like it would without the proper error handling or do I take the consideration to handle the error myself?

I think that both options are good here. This is the kind of situation were you want a panic.
In fact, Rust will perform it. But I've decided to create an Error and handle everything with the usual Result.

Why? Well, I'm trying to replicate a system and good system always have ways to handle exceptions.
It's not like I will be handling everything in a different way than just halting the program, but I want to simulate how the system would know about the problem and be able to perform the exception handling.

One thing to note is that I've decided to name the errors enum with the name `Exception`, my fellow rustaceans will probably suffer a bit looking at it but remember than I'm simulating a system, and those throw exceptions, not errors.
It's not like I've forgotten how Rust considers the errors, but I'm faithful to my simulation.