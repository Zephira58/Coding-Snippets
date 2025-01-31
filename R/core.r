# Basic R Script Covering Main Concepts

# --- Variables and Data Types ---
num <- 42  # Numeric (integer or double)
text <- "Hello, R!"  # Character (string)
bool <- TRUE  # Logical (Boolean)
vec <- c(1, 2, 3, 4, 5)  # Numeric Vector

# --- Lists and Data Frames ---
my_list <- list(name = "Alice", age = 25, scores = c(90, 85, 88))  # List (heterogeneous data)
df <- data.frame(ID = c(1, 2, 3), Name = c("John", "Jane", "Doe"), Score = c(88, 92, 75))  # Data Frame

# --- Conditional Statements ---
if (num > 40) {
  print("Number is greater than 40")
} else {
  print("Number is 40 or less")
}

# --- Loops ---
for (i in 1:5) {
  print(paste("Iteration", i))
}

# While loop example
count <- 1
while (count <= 3) {
  print(paste("Count is", count))
  count <- count + 1
}

# --- Functions ---
square <- function(x) {
  return(x^2)
}
print(square(4))  # Calling the function

# --- Working with Data ---
mean_score <- mean(df$Score)  # Calculate mean of a column
subset_df <- subset(df, Score > 80)  # Filter rows where Score > 80

# --- Plotting ---
plot(vec, main="Basic Plot", xlab="Index", ylab="Value", type="o", col="blue")