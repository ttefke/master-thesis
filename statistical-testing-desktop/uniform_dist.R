# Get all occurrences
obs <- list()

for (i in seq(0, 255)) {
  occurrences = length(which(`all`[["V1"]] == i))
  obs <- append(obs, occurrences)
}

expected <- length((`all`[["V1"]])) / 256
expected <- rep(expected, 256)
chi2 <- sum(((as.numeric(obs) - expected)^2/expected))
print(chi2)
curve(pchisq(x, 255), from=0, to=500)
quantil <- qchisq(0.99, 255)
print(quantil)
if (chi2 < quantil) {
  print("PASSED")
} else {
  print("FAILED")
}

