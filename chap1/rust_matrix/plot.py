import matplotlib.pyplot as plt
import pandas as pd

df = pd.read_csv("bench.csv")

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

# Plot with Legends
plt.plot(df["parameter"], df["mean"], marker='o',label=r'2D')
plt.fill_between(df["parameter"],df["min"],df["max"],alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)

dg = pd.read_csv("bench2.csv")

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

# Plot with Legends
plt.plot(dg["parameter"], dg["mean"], marker='o',label=r'2D (matrixmultiply)')
plt.fill_between(dg["parameter"],dg["min"],dg["max"],alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot2.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Benchmark for matmul", fontsize=16)
plt.xlabel(r'size', fontsize=14)
plt.ylabel(r'time(s)', fontsize=14)

# Plot with Legends
plt.plot(df["parameter"], df["mean"], marker='o',label=r'2D')
plt.fill_between(df["parameter"],df["min"],df["max"],alpha=0.2)
plt.plot(dg["parameter"], dg["mean"], marker='o',label=r'2D (matrixmultiply)')
plt.fill_between(dg["parameter"],dg["min"],dg["max"],alpha=0.2)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot3.png", dpi=300)
