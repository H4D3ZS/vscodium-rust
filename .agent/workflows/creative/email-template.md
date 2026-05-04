---
description: Create responsive HTML email templates that render correctly across email clients
---

# HTML Email Template Creation

I will help you create responsive, cross-client compatible HTML email templates.

## Guardrails
- Use table-based layouts for maximum compatibility
- Inline all CSS styles
- Test across major email clients (Gmail, Outlook, Apple Mail)
- Keep total email size under 100KB

## Steps

### 1. Determine Requirements
Ask clarifying questions:
- What type of email? (transactional, marketing, newsletter)
- What content sections are needed?
- Brand colors and logo?
- Any interactive elements (buttons, links)?

### 2. Create Base Template Structure

Create `emails/<template-name>.html`:
```html
<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xmlns:o="urn:schemas-microsoft-com:office:office">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="x-apple-disable-message-reformatting">
  <title>Email Title</title>
  <!--[if mso]>
  <noscript>
    <xml>
      <o:OfficeDocumentSettings>
        <o:PixelsPerInch>96</o:PixelsPerInch>
      </o:OfficeDocumentSettings>
    </xml>
  </noscript>
  <![endif]-->
  <style>
    /* Reset styles */
    body, table, td, p, a { -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
    table, td { mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
    img { -ms-interpolation-mode: bicubic; border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; }
    body { height: 100% !important; margin: 0 !important; padding: 0 !important; width: 100% !important; }

    /* Responsive styles */
    @media screen and (max-width: 600px) {
      .responsive-table { width: 100% !important; }
      .mobile-padding { padding: 20px !important; }
      .mobile-stack { display: block !important; width: 100% !important; }
    }
  </style>
</head>
<body style="margin: 0; padding: 0; background-color: #f4f4f4;">
  <!-- Email wrapper -->
  <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="100%" style="background-color: #f4f4f4;">
    <tr>
      <td align="center" style="padding: 40px 10px;">
        <!-- Email container -->
        <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="600" class="responsive-table" style="background-color: #ffffff; border-radius: 8px; overflow: hidden;">

          <!-- Header -->
          <tr>
            <td style="padding: 30px; text-align: center; background-color: #4F46E5;">
              <img src="https://via.placeholder.com/150x50" alt="Logo" width="150" style="display: block; margin: 0 auto;">
            </td>
          </tr>

          <!-- Content -->
          <tr>
            <td class="mobile-padding" style="padding: 40px 30px;">
              <h1 style="margin: 0 0 20px; font-family: Arial, sans-serif; font-size: 24px; line-height: 30px; color: #1a1a1a;">
                Your Email Heading
              </h1>
              <p style="margin: 0 0 20px; font-family: Arial, sans-serif; font-size: 16px; line-height: 24px; color: #666666;">
                This is your email content. Keep it concise and focused on the main message.
              </p>

              <!-- CTA Button -->
              <table role="presentation" cellspacing="0" cellpadding="0" border="0" style="margin: 30px 0;">
                <tr>
                  <td style="border-radius: 6px; background-color: #4F46E5;">
                    <a href="https://example.com" target="_blank" style="display: inline-block; padding: 14px 30px; font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; color: #ffffff; text-decoration: none;">
                      Call to Action
                    </a>
                  </td>
                </tr>
              </table>
            </td>
          </tr>

          <!-- Footer -->
          <tr>
            <td style="padding: 30px; text-align: center; background-color: #f8f8f8; border-top: 1px solid #eeeeee;">
              <p style="margin: 0 0 10px; font-family: Arial, sans-serif; font-size: 12px; color: #999999;">
                © 2024 Your Company. All rights reserved.
              </p>
              <p style="margin: 0; font-family: Arial, sans-serif; font-size: 12px; color: #999999;">
                <a href="#" style="color: #666666;">Unsubscribe</a> | <a href="#" style="color: #666666;">Privacy Policy</a>
              </p>
            </td>
          </tr>

        </table>
      </td>
    </tr>
  </table>
</body>
</html>
```

### 3. Add Common Components

**Two-Column Layout:**
```html
<tr>
  <td style="padding: 0 30px;">
    <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="100%">
      <tr>
        <td class="mobile-stack" width="48%" valign="top" style="padding-right: 2%;">
          <h3 style="margin: 0 0 10px; font-family: Arial, sans-serif; font-size: 18px; color: #1a1a1a;">Feature One</h3>
          <p style="margin: 0; font-family: Arial, sans-serif; font-size: 14px; color: #666666;">Description text here.</p>
        </td>
        <td class="mobile-stack" width="48%" valign="top" style="padding-left: 2%;">
          <h3 style="margin: 0 0 10px; font-family: Arial, sans-serif; font-size: 18px; color: #1a1a1a;">Feature Two</h3>
          <p style="margin: 0; font-family: Arial, sans-serif; font-size: 14px; color: #666666;">Description text here.</p>
        </td>
      </tr>
    </table>
  </td>
</tr>
```

**Image with Text:**
```html
<tr>
  <td style="padding: 30px;">
    <img src="https://via.placeholder.com/540x200" alt="Image" width="540" style="display: block; width: 100%; max-width: 540px; height: auto; border-radius: 4px;">
  </td>
</tr>
```

### 4. Outlook-Specific Fixes

For background images in Outlook:
```html
<!--[if mso]>
<v:rect xmlns:v="urn:schemas-microsoft-com:vml" fill="true" stroke="false" style="width:600px;height:300px;">
  <v:fill type="tile" src="https://example.com/bg.jpg" color="#333333"/>
  <v:textbox inset="0,0,0,0">
<![endif]-->
<div style="background-image: url('https://example.com/bg.jpg'); background-size: cover;">
  <!-- Content here -->
</div>
<!--[if mso]>
  </v:textbox>
</v:rect>
<![endif]-->
```

### 5. Test Email

Use these tools to test:
- [Litmus](https://litmus.com) - Cross-client testing
- [Email on Acid](https://emailonacid.com) - Preview across clients
- [Mailtrap](https://mailtrap.io) - Email sandbox

## Email Client Considerations

| Client | Key Issues |
|--------|-----------|
| **Outlook** | Uses Word rendering engine, limited CSS support |
| **Gmail** | Strips `<style>` tags, requires inline styles |
| **Apple Mail** | Generally good support |
| **Yahoo** | Strips some CSS, test thoroughly |

## Guidelines
- Always include plain-text version
- Use web-safe fonts (Arial, Georgia, Times New Roman)
- Keep subject lines under 50 characters
- Include preheader text
- Test on mobile devices

## Reference
- [Can I Email](https://www.caniemail.com/) - CSS support reference
- [Email Client Market Share](https://www.litmus.com/email-client-market-share)
