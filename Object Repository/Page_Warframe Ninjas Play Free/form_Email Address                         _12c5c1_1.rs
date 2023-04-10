<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_Email Address                         _12c5c1_1</name>
   <tag></tag>
   <elementGuidId>77b9016e-d6a2-4624-a082-bfc32bbcf9b1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@action='https://www.warframe.com/signup']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>form.signup-form</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>form</value>
      <webElementGuid>10942279-c2f7-4256-8ad4-8d55a6ab7117</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>action</name>
      <type>Main</type>
      <value>https://www.warframe.com/signup</value>
      <webElementGuid>6d3702b0-f2f4-46ad-a721-2dc7e6af18a5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>d6381bb8-1708-429c-92f0-3bf3f7c15e7c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>signup-form</value>
      <webElementGuid>4b07715e-24d9-4967-97fc-56cdf6e75c94</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onsubmit</name>
      <type>Main</type>
      <value>return false</value>
      <webElementGuid>d11360fa-73b9-4fc0-bc8b-e6f7fadf6a56</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>novalidate</name>
      <type>Main</type>
      <value>novalidate</value>
      <webElementGuid>435e4094-9b1e-467b-a9b7-e2743427e045</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
                Email Address:
                
            

            
                Confirm Email:
                
            Invalid Email Address

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            

            
                
                I would like to receive Warframe news, special offers, and more. (This setting can be changed in Account Management at any time.)
            

            
                
                I am over the age of 16 and agree to the Terms of Use &amp; Privacy Policy or have parental consent.
            

                                                            
    
        window.de_recaptcha_ready = window.de_recaptcha_ready || false;

        document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
            const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
            recaptchaSubmitElems.forEach(function(elem) {
                elem.disabled = true;
            });

            
        });

        function recaptchaReady() {
            grecaptcha.enterprise.ready(function() {
                window.de_recaptcha_ready = true;

                const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
                recaptchaSubmitElems.forEach(function(elem) {
                    elem.disabled = false;
                });

                            });
        }

        function loadRecaptcha() {
            let recaptchaScript = document.querySelector('#recaptchaScript');
            if(!recaptchaScript.src) {
                document.querySelector('#recaptchaScript').src = &quot;https://www.recaptcha.net/recaptcha/enterprise.js?onload=recaptchaReady&amp;render=6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;;
            }
        }

        async function fetchRecaptchaToken(action) {
            let container = document.getElementById('recaptcha-'+action)

            if(window.de_recaptcha_ready) {
                await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action }).then(function(token) {
                    let elem = container.querySelector('.g-recaptcha-response');
                    elem.innerHTML = token;
                });
                return true;
            } else {
                await grecaptcha.enterprise.ready(async function () {
                    await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action}).then(function (token) {
                        let elem = container.querySelector('.g-recaptcha-response');
                        elem.innerHTML = token;
                    });
                });
                return false;
            }
        }

    

    

            
            
                
                    Join Now
                
            
            
        </value>
      <webElementGuid>39a9689e-c149-44e5-ad56-6e362a80632a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;signupLightbox&quot;)/div[@class=&quot;lightboxForm signup&quot;]/div[@class=&quot;formHolder&quot;]/div[@class=&quot;container content signupContent&quot;]/div[@class=&quot;row pcSignup&quot;]/form[@class=&quot;signup-form&quot;]</value>
      <webElementGuid>379e5211-ff47-4ebd-9b04-a8f5dc222dfa</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//form[@action='https://www.warframe.com/signup']</value>
      <webElementGuid>290b3552-9f19-4ce1-a6f3-9dae9fa5022a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='signupLightbox']/div/div/div/div[3]/form</value>
      <webElementGuid>471e35e2-94c9-4056-b087-a8c318a95649</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Nintendo Switch'])[1]/following::form[1]</value>
      <webElementGuid>c4e8dc0d-7e03-4f6e-b895-19a61c4052aa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Xbox'])[1]/following::form[1]</value>
      <webElementGuid>060b259e-26d4-49a7-ad47-14e95c5ec053</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form</value>
      <webElementGuid>84f97d38-9556-46fe-bc77-a1a622d0d32e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//form[(text() = concat(&quot;
            
                Email Address:
                
            

            
                Confirm Email:
                
            Invalid Email Address

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            

            
                
                I would like to receive Warframe news, special offers, and more. (This setting can be changed in Account Management at any time.)
            

            
                
                I am over the age of 16 and agree to the Terms of Use &amp; Privacy Policy or have parental consent.
            

                                                            
    
        window.de_recaptcha_ready = window.de_recaptcha_ready || false;

        document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
            const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
            recaptchaSubmitElems.forEach(function(elem) {
                elem.disabled = true;
            });

            
        });

        function recaptchaReady() {
            grecaptcha.enterprise.ready(function() {
                window.de_recaptcha_ready = true;

                const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
                recaptchaSubmitElems.forEach(function(elem) {
                    elem.disabled = false;
                });

                            });
        }

        function loadRecaptcha() {
            let recaptchaScript = document.querySelector(&quot; , &quot;'&quot; , &quot;#recaptchaScript&quot; , &quot;'&quot; , &quot;);
            if(!recaptchaScript.src) {
                document.querySelector(&quot; , &quot;'&quot; , &quot;#recaptchaScript&quot; , &quot;'&quot; , &quot;).src = &quot;https://www.recaptcha.net/recaptcha/enterprise.js?onload=recaptchaReady&amp;render=6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;;
            }
        }

        async function fetchRecaptchaToken(action) {
            let container = document.getElementById(&quot; , &quot;'&quot; , &quot;recaptcha-&quot; , &quot;'&quot; , &quot;+action)

            if(window.de_recaptcha_ready) {
                await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action }).then(function(token) {
                    let elem = container.querySelector(&quot; , &quot;'&quot; , &quot;.g-recaptcha-response&quot; , &quot;'&quot; , &quot;);
                    elem.innerHTML = token;
                });
                return true;
            } else {
                await grecaptcha.enterprise.ready(async function () {
                    await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action}).then(function (token) {
                        let elem = container.querySelector(&quot; , &quot;'&quot; , &quot;.g-recaptcha-response&quot; , &quot;'&quot; , &quot;);
                        elem.innerHTML = token;
                    });
                });
                return false;
            }
        }

    

    

            
            
                
                    Join Now
                
            
            
        &quot;) or . = concat(&quot;
            
                Email Address:
                
            

            
                Confirm Email:
                
            Invalid Email Address

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            

            
                
                I would like to receive Warframe news, special offers, and more. (This setting can be changed in Account Management at any time.)
            

            
                
                I am over the age of 16 and agree to the Terms of Use &amp; Privacy Policy or have parental consent.
            

                                                            
    
        window.de_recaptcha_ready = window.de_recaptcha_ready || false;

        document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
            const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
            recaptchaSubmitElems.forEach(function(elem) {
                elem.disabled = true;
            });

            
        });

        function recaptchaReady() {
            grecaptcha.enterprise.ready(function() {
                window.de_recaptcha_ready = true;

                const recaptchaSubmitElems = document.querySelectorAll(&quot;.uses-recaptcha&quot;);
                recaptchaSubmitElems.forEach(function(elem) {
                    elem.disabled = false;
                });

                            });
        }

        function loadRecaptcha() {
            let recaptchaScript = document.querySelector(&quot; , &quot;'&quot; , &quot;#recaptchaScript&quot; , &quot;'&quot; , &quot;);
            if(!recaptchaScript.src) {
                document.querySelector(&quot; , &quot;'&quot; , &quot;#recaptchaScript&quot; , &quot;'&quot; , &quot;).src = &quot;https://www.recaptcha.net/recaptcha/enterprise.js?onload=recaptchaReady&amp;render=6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;;
            }
        }

        async function fetchRecaptchaToken(action) {
            let container = document.getElementById(&quot; , &quot;'&quot; , &quot;recaptcha-&quot; , &quot;'&quot; , &quot;+action)

            if(window.de_recaptcha_ready) {
                await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action }).then(function(token) {
                    let elem = container.querySelector(&quot; , &quot;'&quot; , &quot;.g-recaptcha-response&quot; , &quot;'&quot; , &quot;);
                    elem.innerHTML = token;
                });
                return true;
            } else {
                await grecaptcha.enterprise.ready(async function () {
                    await grecaptcha.enterprise.execute(&quot;6LcWYwYgAAAAAIw9zG71CAPMr2oJPm3zpiaCXLVj&quot;, {action: action}).then(function (token) {
                        let elem = container.querySelector(&quot; , &quot;'&quot; , &quot;.g-recaptcha-response&quot; , &quot;'&quot; , &quot;);
                        elem.innerHTML = token;
                    });
                });
                return false;
            }
        }

    

    

            
            
                
                    Join Now
                
            
            
        &quot;))]</value>
      <webElementGuid>9a270d09-f821-411a-8733-903bed863ba8</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
