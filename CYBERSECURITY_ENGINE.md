# 🛡️ AIRI Cybersecurity Engine - Usage Guide

## What It Protects Against

### 1. **Port Scanning Detection** 🔍
- Detects Nmap, masscan, and other port scanning tools
- Monitors for unusual connection patterns
- Alerts when someone is probing your system

### 2. **Phishing URL Detection** 🎣
- Analyzes URLs for phishing indicators
- Checks for suspicious TLDs (.tk, .ml, .ga, etc.)
- Detects brand impersonation (paypal, microsoft, etc.)
- Identifies URL shorteners hiding destinations
- Warns about non-HTTPS sensitive sites

### 3. **Social Engineering Detection** 🎭
- Analyzes email/text content for manipulation tactics
- Detects urgency tactics ("act now!", "limited time!")
- Identifies fear-based manipulation ("account suspended!")
- Catches greed tactics ("you won!", "free gift!")
- Spots requests for sensitive information

### 4. **Network Intrusion Monitoring** 🚨
- Monitors for unusual network activity
- Detects potential unauthorized access
- Alerts on suspicious connection patterns

---

## How to Use

### Check a Suspicious URL

**In Browser Console (F12)**:
```javascript
// Quick URL check
airiCybersecurity.quickURLCheck('https://suspicious-site.tk/login');

// Detailed analysis
const analysis = airiCybersecurity.analyzeURL('https://paypal-verify-account.xyz/verify');
console.log(analysis);
```

**Example Output**:
```
🛡️ CYBERSECURITY THREAT DETECTED:
   Type: phishing
   Severity: high
   Source: https://suspicious-site.tk/login
   Details: Phishing risk score: 85/100. Indicators: Suspicious TLD: .tk, Brand impersonation: paypal, Non-HTTPS for sensitive site
   Blocked: true
```

---

### Check an Email/Message for Phishing

**In Browser Console**:
```javascript
// Quick content check
const email = `
Dear Customer,

Your account has been suspended due to suspicious activity.
Click here immediately to verify your identity: bit.ly/verify-now

If you don't act within 24 hours, your account will be permanently locked.

Please provide:
- Your password
- Credit card number
- Social Security Number

Best regards,
Security Team
`;

airiCybersecurity.quickContentCheck(email, 'Email from "Security Team"');

// Detailed analysis
const analysis = airiCybersecurity.analyzeContent(email);
console.log(analysis);
```

**Example Output**:
```
🛡️ CYBERSECURITY THREAT DETECTED:
   Type: social_engineering
   Severity: high
   Source: Email from "Security Team"
   Details: Phishing confidence: 95/100. Indicators: Urgency tactic: immediately, Fear tactic: account suspended, Requesting sensitive info: password, Spelling error detected
   Blocked: false
```

---

### Manual Security Scan

Scan AIRI's recent thoughts for security threats:

```javascript
// In browser console
const scanResult = await airiSafetyProtocol.performSecurityScan();
console.log(scanResult);
```

**Output**:
```
[Safety] 🔍 Performing manual security scan...
[Safety] Scan complete: ✅ SECURE
// OR
[Safety] Scan complete: ⚠️ 2 threats detected
```

---

### View Threat History

```javascript
// Get last 20 threats
const threats = airiCybersecurity.getThreatHistory(20);
console.log(threats);

// Get detailed threat info
threats.forEach(threat => {
    console.log(`[${threat.severity}] ${threat.type}: ${threat.details}`);
});
```

---

## Risk Score Interpretation

### URL Analysis
| Score | Risk Level | Action |
|-------|------------|--------|
| 0-29 | ✅ Low | Safe to visit |
| 30-49 | ⚠️ Medium | Proceed with caution |
| 50-69 | ⚠️ High | Verify before clicking |
| 70-100 | 🚨 Critical | DO NOT CLICK - Likely phishing |

### Content Analysis
| Confidence | Likelihood | Action |
|------------|------------|--------|
| 0-19 | ✅ Unlikely | Probably safe |
| 20-39 | ⚠️ Possible | Be cautious |
| 40-69 | ⚠️ Likely | Probably phishing |
| 70-100 | 🚨 Certain | Definitely phishing |

---

## Common Phishing Indicators

### URL Red Flags 🚩
- Suspicious TLDs: `.tk`, `.ml`, `.ga`, `.cf`, `.gq`
- IP addresses instead of domains
- Too many subdomains: `secure.login.verify.account.site.com`
- Brand names in wrong places: `paypal-security.com` (not `paypal.com`)
- URL shorteners: `bit.ly`, `tinyurl.com` (hiding real destination)
- Non-HTTPS on login/banking sites

### Content Red Flags 🚩
- **Urgency**: "Act now!", "Within 24 hours!", "Limited time!"
- **Threats**: "Account suspended", "Unauthorized access", "Will be closed"
- **Greed**: "You won!", "Congratulations", "Free gift card"
- **Sensitive Requests**: Asking for passwords, credit cards, SSN
- **Generic Greetings**: "Dear Customer", "Dear User"
- **Spelling Errors**: "recieve", "occured", "seperate"

---

## Integration Examples

### Check Links Before Clicking

Add to your browser's context menu or use as a habit:

```javascript
// Right-click on link → Inspect → Console
const link = document.querySelector('a:hover');
if (link) {
    const href = link.href;
    console.log('Checking link:', href);
    airiCybersecurity.quickURLCheck(href);
}
```

### Verify Email Before Responding

```javascript
// Copy email text, paste into console
const emailText = `...paste email here...`;
const analysis = airiCybersecurity.analyzeContent(emailText);

if (analysis.isPhishing) {
    console.warn('⚠️ PHISHING DETECTED! Do not respond!');
    console.log('Indicators:', analysis.indicators);
} else {
    console.log('✅ Email appears safe');
}
```

---

## Real-World Examples

### Example 1: PayPal Phishing
```javascript
const url = 'https://paypal-security-verify.tk/login';
const analysis = airiCybersecurity.analyzeURL(url);

console.log(analysis);
// Output:
// {
//   isPhishing: true,
//   riskScore: 90,
//   indicators: [
//     'Suspicious TLD: .tk',
//     'Brand impersonation: paypal',
//     'Suspicious keyword: security',
//     'Suspicious keyword: verify',
//     'Suspicious keyword: login'
//   ],
//   recommendation: '⚠️ HIGH RISK - Do not click! Likely phishing attempt.'
// }
```

### Example 2: Fake Security Email
```javascript
const email = `
From: security@microsoft-support.xyz
Subject: Urgent: Your Account Has Been Compromised

Dear Microsoft User,

We detected suspicious activity on your account.
Your account will be suspended within 24 hours unless you verify immediately.

Click here to secure your account: bit.ly/ms-verify

Please provide:
- Email password
- Recovery phone number
- Last 4 digits of credit card

Microsoft Security Team
`;

const analysis = airiCybersecurity.analyzeContent(email);
console.log(analysis);
// Output:
// {
//   isPhishing: true,
//   confidence: 95,
//   indicators: [
//     'Urgency tactic: urgent',
//     'Urgency tactic: immediately',
//     'Fear tactic: account has been compromised',
//     'Fear tactic: suspended within 24 hours',
//     'Requesting sensitive info: password',
//     'URL shortener detected',
//     'Generic greeting: dear microsoft user'
//   ]
// }
```

---

## Automatic Monitoring

The cybersecurity engine runs automatically in the background:

- ✅ **Port scan detection** every 10 seconds
- ✅ **Network monitoring** continuous
- ✅ **Threat alerts** real-time via safety protocol
- ✅ **Threat history** logged for review

---

## Keyboard Shortcuts

```javascript
// Quick URL check (add to your browser bookmarks)
javascript:(function(){
    const url = prompt('Enter URL to check:');
    if (url) airiCybersecurity.quickURLCheck(url);
})();
```

---

**Status**: ✅ Active and monitoring  
**Protection Level**: Maximum  
**Last Scan**: Continuous

🛡️ **AIRI is your cybersecurity guardian!**
