<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_Email Address                         _0ae75f</name>
   <tag></tag>
   <elementGuidId>581c9e9f-0312-48d0-8ab3-48e1236a5d25</elementGuidId>
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
      <webElementGuid>41d04044-b7e3-4743-b74d-de174246d615</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>action</name>
      <type>Main</type>
      <value>https://www.warframe.com/signup</value>
      <webElementGuid>d0ea884e-61a0-402a-809c-33bc129f8853</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>be52f669-12f4-47ce-afcf-6deb7840060c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>signup-form</value>
      <webElementGuid>91127472-f886-4733-b809-db343cf3574e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onsubmit</name>
      <type>Main</type>
      <value>return false</value>
      <webElementGuid>9909bf18-9115-4a84-9eb5-a7bba85c3df3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>novalidate</name>
      <type>Main</type>
      <value>novalidate</value>
      <webElementGuid>e2db0bd2-deb9-4970-97c7-9d602e4bf359</webElementGuid>
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
      <webElementGuid>f0273058-ab02-4db4-81cb-22ee8a985e8e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;signupLightbox&quot;)/div[@class=&quot;lightboxForm signup&quot;]/div[@class=&quot;formHolder&quot;]/div[@class=&quot;container content signupContent&quot;]/div[@class=&quot;row pcSignup&quot;]/form[@class=&quot;signup-form&quot;]</value>
      <webElementGuid>e0228254-7e9f-4db8-9911-ec4d3e2a1cc1</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//form[@action='https://www.warframe.com/signup']</value>
      <webElementGuid>a41dfcc1-394e-418a-8e19-170d29c1ae70</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='signupLightbox']/div/div/div/div[3]/form</value>
      <webElementGuid>1457a080-2c25-4efb-b278-c4b938a64117</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Nintendo Switch'])[1]/following::form[1]</value>
      <webElementGuid>cb4c42cd-b4f7-4956-8c9d-3fea0fb56937</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Xbox'])[1]/following::form[1]</value>
      <webElementGuid>6e3acc12-1b09-4cce-bdb5-9e4a55dbeef8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form</value>
      <webElementGuid>643c6661-3036-457a-8a93-0ea3046547bc</webElementGuid>
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
      <webElementGuid>9f08eb91-0971-4680-b1d1-72b574edb69a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
