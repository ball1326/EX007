<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>form_Email Address                         _12c5c1</name>
   <tag></tag>
   <elementGuidId>fde97d72-7139-4b26-9b3b-b173f2d9e50a</elementGuidId>
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
      <webElementGuid>159d0903-cef1-42e5-8367-d197295a0912</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>action</name>
      <type>Main</type>
      <value>https://www.warframe.com/signup</value>
      <webElementGuid>96b2252e-a5af-4ab1-adc5-b4cc85d22d34</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>method</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>e946527d-7e23-4364-bd36-250534612555</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>signup-form</value>
      <webElementGuid>121d605f-5041-4340-bc33-129885313a62</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onsubmit</name>
      <type>Main</type>
      <value>return false</value>
      <webElementGuid>c4c1abf8-64c9-4436-abd4-bbd88b430f8e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>novalidate</name>
      <type>Main</type>
      <value>novalidate</value>
      <webElementGuid>d357462d-ace6-421c-9d08-d2d9e9d6f641</webElementGuid>
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
      <webElementGuid>693f9928-a1dd-4861-9b38-5e537becebf3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;signupLightbox&quot;)/div[@class=&quot;lightboxForm signup&quot;]/div[@class=&quot;formHolder&quot;]/div[@class=&quot;container content signupContent&quot;]/div[@class=&quot;row pcSignup&quot;]/form[@class=&quot;signup-form&quot;]</value>
      <webElementGuid>5cc5aa7d-2a58-42d3-82d3-03eba906b45e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//form[@action='https://www.warframe.com/signup']</value>
      <webElementGuid>bf96a914-9406-4cb3-908d-8a0616fd0c0a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='signupLightbox']/div/div/div/div[3]/form</value>
      <webElementGuid>a7c5a982-5a74-4fbe-b0f7-912b094d2e01</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Nintendo Switch'])[1]/following::form[1]</value>
      <webElementGuid>405f6eeb-efb7-4ded-9302-c7a6c5041526</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Xbox'])[1]/following::form[1]</value>
      <webElementGuid>17b9232b-6c1d-4f15-9ea0-11e7e6ce199f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//form</value>
      <webElementGuid>63ac6ab9-4258-4c61-b579-a22ba26ebc65</webElementGuid>
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
      <webElementGuid>0690657b-e1ca-4d44-8b41-6d7369d32524</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
