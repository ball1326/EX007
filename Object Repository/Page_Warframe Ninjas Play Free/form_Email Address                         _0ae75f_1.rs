<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_Email Address                         _0ae75f_1</name>
   <tag></tag>
   <elementGuidId>4bd5b655-3133-47a6-8856-11f4403f0fdd</elementGuidId>
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
      <webElementGuid>89ea6427-de17-4507-9f4a-0d604a828d72</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>action</name>
      <type>Main</type>
      <value>https://www.warframe.com/signup</value>
      <webElementGuid>bf742768-756a-4b40-97e1-3e427d705849</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>40d34ed2-7a4d-4172-bcdc-a102054bfe5f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>signup-form</value>
      <webElementGuid>0ff3254d-6031-4e2a-b1d0-2056a9fc0a18</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onsubmit</name>
      <type>Main</type>
      <value>return false</value>
      <webElementGuid>e44c9fbe-dc6e-4318-89db-f9aa8ca451d6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>novalidate</name>
      <type>Main</type>
      <value>novalidate</value>
      <webElementGuid>701e1554-35a0-470c-baca-a5b4ee63ef36</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
                Email Address:
                
            

            
                Confirm Email:
                
            

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            Your confirmed password doesn't match.

            
                
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
      <webElementGuid>da62e61c-861a-4ef9-a073-c04de4a298b7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;signupLightbox&quot;)/div[@class=&quot;lightboxForm signup&quot;]/div[@class=&quot;formHolder&quot;]/div[@class=&quot;container content signupContent&quot;]/div[@class=&quot;row pcSignup&quot;]/form[@class=&quot;signup-form&quot;]</value>
      <webElementGuid>558098e2-b3bf-44a2-b26f-d7e94b4a078f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//form[@action='https://www.warframe.com/signup']</value>
      <webElementGuid>8426d0dc-38e3-4a7e-a178-58e97cc8e42d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='signupLightbox']/div/div/div/div[3]/form</value>
      <webElementGuid>8b002706-1c79-4ec4-80bf-09675de5ed8c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Nintendo Switch'])[1]/following::form[1]</value>
      <webElementGuid>e41cb1bc-3a00-4f9d-a65b-d61a56574f67</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Xbox'])[1]/following::form[1]</value>
      <webElementGuid>44145bd0-f90a-4fbc-ba25-74c7ee9b58b7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form</value>
      <webElementGuid>80a368ce-4507-4a85-bd5f-410e4d7b6afd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//form[(text() = concat(&quot;
            
                Email Address:
                
            

            
                Confirm Email:
                
            

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            Your confirmed password doesn&quot; , &quot;'&quot; , &quot;t match.

            
                
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
                
            

            
                Alias:
                
                            

            
                Password:
                
            Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.

            
                A strong password:
                
                    Is between 8 and 40 characters long.
                    Contains at least 1 lowercase and uppercase letter.
                    Contains at least a number and a symbol.
                
            

            
                Confirm Password:
                
            Your confirmed password doesn&quot; , &quot;'&quot; , &quot;t match.

            
                
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
      <webElementGuid>bde0b930-bdda-4162-b56a-7533776cb2bd</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
