What the below is really describing is profiled motion planning for a 1 dimensional mechanism plus an added bonus of keeping the drivetrain straight. Most things in () are generalizing the problem to 1D mechanisms

# Input:
- Drivetrain (Mechanism to tell to do something)
- Odom (Where the mechanism is)
- Drivetrain Parameters (Also where the mechanism is?)
  - encoderTicksPerRev; 
  - gearRatio;
  - wheelCircumference;
- Rotational Controller (extra controller for keeping drivetrain straight)
- Distance to Drive (distance to move the mechanism from where it is)
- Max Velocity 
- Max Acceleration
- Slow Down Coefficient (multiplied by Acceleration Distance In to allow an early or late slow down)

# Members:
- ALL INPUTS
- Starting Pose 
- Timer
- Acceleration Distance In (how long in distance you are in when you get to max vel)
- Speed from previous iteration
- Time from previous iteration

# Initialization:
1. Set speed and time from previous iteration
2. Start the timer
3. Reset drive encoders (assume we are where we want to be. Needed for simple implementation but can introduce error) (Tell mechanism it is at 0, may not be necessary)
4. *Get the starting pose from the odometry (only used for angle correction in drivebase)*
5. Calculate the Acceleration Distance In

# Periodic Loop:
1. Calculate $delta$ time
2. *Get the current pose (only used for angle correction in drivebase)*
3. *Calculate rotational error (only used for angle correction in drivebase)*
5. Calculate the target speed (either the former speed plus acceleration or max speed)
4. Calculate the remaining distance to drive (remaining distance to move)
6. If in deceleration zone, clamp the speed to sqrt(Remaining Distance * Max Accel) or target speed, whichever is lower
   1. Remaining Distance * Max Accel gives you your max decell speed, sqrt() simply adjusts the decell curve.
7. *Calculate the turning offset: Theta_PID(Theta_Error) * speed (only used for angle correction in drivebase)*
8. *Left Speed = speed + turning offset (only used for angle correction in drivebase)*
9.  *Right Speed = speed - turning offset (only used for angle correction in drivebase)*
10. Update last speed & time
11. Set the mechanism velocity

# Rustify This:
There needs to be some saved data between iterations. Could be passed in
-  prev time & speed

Break it into three different methods: 
- Not at max speed yet (AccelerateProfiled)
  - Takes in:
    - previous time and speed
    - max accel
  - Does: Simple Math
  - Returns: Velocity
- At max speed but not in decell zone (just keep running at max speed)
  - Takes in: Nothing
  - Does: Nothing
  - Returns: Max Speed (velocity)
- In decell zone (DecelerateProfiled)
  - Takes in:
    - remaining distance
    - Max Accel
    - previous time and speed (It does this to min(speed, maxDecell), but I think speed will always be greater than maxDecell. So I don't think it needs this.)
  - Does: Simple Math
  - Returns: Velocity

In the above case, there needs to be a controller controlling the methods. Every iteration it needs to calculate the remaining distance and know the previous time and speed. At the top level, it should be one line of code that starts all of this. That line can go into a loop that ends when the mechanism gets to it's desired state. We will figure out concurency later. Previous time and speed can just be local variables in that method. Time to break out of the OOP nightmare. 