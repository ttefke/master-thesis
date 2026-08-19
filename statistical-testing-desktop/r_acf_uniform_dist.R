device <- "chaoskey"

# ACF
lag <- 10000
coeff8 <- acf(`all`[["V1"]], lag=lag)
pdf(paste("/tmp/",device,"full_acf.pdf"), width=8, height=6)
plot(coeff8[1:lag], main=paste("ACF of observed ChaosKey random output"))
dev.off()

max(coeff8[["acf"]][2:length(coeff8[["acf"]])])

# Uniform dist
obs8 <- table(all)
chisq8 <- chisq.test(obs8, p=rep(1/256, 256))
print(chisq8)
maxchisq8 <- qchisq(0.99, 255)
print(maxchisq8)
print("Passed: ")
chisq8[["statistic"]] < maxchisq8

pdf(paste("/tmp/",device,"_hist.pdf"), width=8, height=6)
hist(`all`[["V1"]],
     breaks=seq(min(`all`[["V1"]]), max(`all`[["V1"]]), length.out = 257),
     freq=FALSE,
     xlab="Observed output",
     main=paste("Histogram of observed ChaosKey random output"))
dev.off()
# 
