<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Close                                  _1559d9</name>
   <tag></tag>
   <elementGuidId>74fd1a8c-1350-435b-8212-201c652d25dd</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.lightboxOverlay</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>828dc84e-a1de-4492-9f2f-4ac1612ffe19</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>lightboxOverlay</value>
      <webElementGuid>512424dc-f7a6-4f24-9311-ba4b5768c9b3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

    
        
        Close
    

        
            
                
                
                                            
    
        
        
        
        What will you be playing on?
    
    
        
            
            PC
        
        
            
            PlayStation
        
        
            
            Xbox
        
        
            
            Nintendo Switch
        
    
    
        
            
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
                
            
            
        
    


    window.dataLayer = window.dataLayer || [];

    
    $(document).ready(function(){

        $.validator.addMethod(&quot;pwstrength&quot;, function(value) {
            let testsPassed = 0;
            if(/[a-z]/.test(value)) testsPassed++; // has a lowercase letter
            if(/[A-Z]/.test(value)) testsPassed++; // has an uppercase letter
            if(/\d/.test(value)) testsPassed++; // has a digit
            if(/[^\dA-Za-z]/.test(value)) testsPassed++; // has a character that is not a letter or number
            return (testsPassed >= 3);
        });

        

        $('.aliasOption').click(function() {
            let inputField = $('#alias input:text')[0];
            $(inputField).val($(this).text());
        });

        $(&quot;.signup-form&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                email_confirm: {
                    required: true,
                    equalTo: '.signup-form [name=&quot;email&quot;]'
                },
                alias: {
                    required: true,
                    minlength: 4,
                },
                password: {
                    required: true,
                    minlength: 8,
                    maxlength: 40,
                    pwstrength: true
                },
                password_confirm :{
                    required: true,
                    equalTo: '.signup-form [name=&quot;password&quot;]',
                },
                accept_tos: {
                    required: true
                },
            },
            messages: {
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                },
                email_confirm: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;,
                    equalTo: &quot;Your confirmed email doesn&amp;#039;t match.&quot;
                },
                alias: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Invalid alias (only letters, numbers, periods, under-scores, and hyphens)&quot;,
                },
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Weak Password:  Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    maxlength: &quot;Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    pwstrength: &quot;Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                },
                password_confirm: {
                    required: &quot;This field is required.&quot;,
                    equalTo: &quot;Your confirmed password doesn&amp;#039;t match.&quot;,
                },
                accept_tos: {
                    required: &quot;This field is required.&quot;,
                },
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass('submitted')){
                    return;
                }
                $(form).addClass('submitted');
                $('.uses-recaptcha').prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken('signup');

                form.submit();
            }
        });

        $('.signupPlat.pc').click(function(e){
            e.preventDefault();

            dataLayer.push({
                &quot;event&quot;: &quot;account-signup&quot;,
                &quot;eventCategory&quot;: &quot;account-signup&quot;,
                &quot;eventAction&quot;: &quot;open-lightbox&quot;,
            });

            $('#signupPlatforms,.whatWill').fadeOut(200,function(){
                $('.pcSignup').fadeIn(200);
            });
        });

        
    });

                                    
            
        

        
            
                
                
                                            
    
        
            
                
                
                
                Log In With
            
            
                
                
                
            
            
                
                
                
                or
            
            
                                
                    Email Address:
                    
                

                
                    Password:
                    
                
                
                    Remember Me
                
                Forgot Password
                
                
                    
                        
                            
                        
                    
                
                
                
                
    
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

    

    

            
            
                
                
                
                or
            
            
                
                    Sign Up
                
            
        
    


    $(document).ready(function () {
        $(&quot;form#loginForm&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                password: {
                    required: true,
                    minlength: 4
                },
            },
            messages: {
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;You must enter a valid password.&quot;
                },
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                }
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass('submitted')){
                    return;
                }
                $(form).addClass('submitted');
                $('.uses-recaptcha').prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken('login');

                form.submit();
            }
        });
    });

                                    
            
        

</value>
      <webElementGuid>12bfd11c-1d39-4784-a9a2-97d3a40eb620</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;wf-michroma-n4-active wf-opensans-n7-active wf-opensans-n3-active wf-opensans-n4-active wf-opensans-i3-active wf-opensans-i4-active wf-opensans-i7-active wf-active&quot;]/body[@class=&quot;landing lang-en logged-out&quot;]/div[@class=&quot;lightboxOverlay&quot;]</value>
      <webElementGuid>c6589e11-c14e-4d07-a1b1-8d0e72148140</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>b34ff43d-6d7b-4676-ba7d-9eafa80a0061</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;

    
        
        Close
    

        
            
                
                
                                            
    
        
        
        
        What will you be playing on?
    
    
        
            
            PC
        
        
            
            PlayStation
        
        
            
            Xbox
        
        
            
            Nintendo Switch
        
    
    
        
            
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
                
            
            
        
    


    window.dataLayer = window.dataLayer || [];

    
    $(document).ready(function(){

        $.validator.addMethod(&quot;pwstrength&quot;, function(value) {
            let testsPassed = 0;
            if(/[a-z]/.test(value)) testsPassed++; // has a lowercase letter
            if(/[A-Z]/.test(value)) testsPassed++; // has an uppercase letter
            if(/\d/.test(value)) testsPassed++; // has a digit
            if(/[^\dA-Za-z]/.test(value)) testsPassed++; // has a character that is not a letter or number
            return (testsPassed >= 3);
        });

        

        $(&quot; , &quot;'&quot; , &quot;.aliasOption&quot; , &quot;'&quot; , &quot;).click(function() {
            let inputField = $(&quot; , &quot;'&quot; , &quot;#alias input:text&quot; , &quot;'&quot; , &quot;)[0];
            $(inputField).val($(this).text());
        });

        $(&quot;.signup-form&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                email_confirm: {
                    required: true,
                    equalTo: &quot; , &quot;'&quot; , &quot;.signup-form [name=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;
                },
                alias: {
                    required: true,
                    minlength: 4,
                },
                password: {
                    required: true,
                    minlength: 8,
                    maxlength: 40,
                    pwstrength: true
                },
                password_confirm :{
                    required: true,
                    equalTo: &quot; , &quot;'&quot; , &quot;.signup-form [name=&quot;password&quot;]&quot; , &quot;'&quot; , &quot;,
                },
                accept_tos: {
                    required: true
                },
            },
            messages: {
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                },
                email_confirm: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;,
                    equalTo: &quot;Your confirmed email doesn&amp;#039;t match.&quot;
                },
                alias: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Invalid alias (only letters, numbers, periods, under-scores, and hyphens)&quot;,
                },
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Weak Password:  Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    maxlength: &quot;Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    pwstrength: &quot;Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                },
                password_confirm: {
                    required: &quot;This field is required.&quot;,
                    equalTo: &quot;Your confirmed password doesn&amp;#039;t match.&quot;,
                },
                accept_tos: {
                    required: &quot;This field is required.&quot;,
                },
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;)){
                    return;
                }
                $(form).addClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;.uses-recaptcha&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken(&quot; , &quot;'&quot; , &quot;signup&quot; , &quot;'&quot; , &quot;);

                form.submit();
            }
        });

        $(&quot; , &quot;'&quot; , &quot;.signupPlat.pc&quot; , &quot;'&quot; , &quot;).click(function(e){
            e.preventDefault();

            dataLayer.push({
                &quot;event&quot;: &quot;account-signup&quot;,
                &quot;eventCategory&quot;: &quot;account-signup&quot;,
                &quot;eventAction&quot;: &quot;open-lightbox&quot;,
            });

            $(&quot; , &quot;'&quot; , &quot;#signupPlatforms,.whatWill&quot; , &quot;'&quot; , &quot;).fadeOut(200,function(){
                $(&quot; , &quot;'&quot; , &quot;.pcSignup&quot; , &quot;'&quot; , &quot;).fadeIn(200);
            });
        });

        
    });

                                    
            
        

        
            
                
                
                                            
    
        
            
                
                
                
                Log In With
            
            
                
                
                
            
            
                
                
                
                or
            
            
                                
                    Email Address:
                    
                

                
                    Password:
                    
                
                
                    Remember Me
                
                Forgot Password
                
                
                    
                        
                            
                        
                    
                
                
                
                
    
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

    

    

            
            
                
                
                
                or
            
            
                
                    Sign Up
                
            
        
    


    $(document).ready(function () {
        $(&quot;form#loginForm&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                password: {
                    required: true,
                    minlength: 4
                },
            },
            messages: {
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;You must enter a valid password.&quot;
                },
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                }
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;)){
                    return;
                }
                $(form).addClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;.uses-recaptcha&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken(&quot; , &quot;'&quot; , &quot;login&quot; , &quot;'&quot; , &quot;);

                form.submit();
            }
        });
    });

                                    
            
        

&quot;) or . = concat(&quot;

    
        
        Close
    

        
            
                
                
                                            
    
        
        
        
        What will you be playing on?
    
    
        
            
            PC
        
        
            
            PlayStation
        
        
            
            Xbox
        
        
            
            Nintendo Switch
        
    
    
        
            
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
                
            
            
        
    


    window.dataLayer = window.dataLayer || [];

    
    $(document).ready(function(){

        $.validator.addMethod(&quot;pwstrength&quot;, function(value) {
            let testsPassed = 0;
            if(/[a-z]/.test(value)) testsPassed++; // has a lowercase letter
            if(/[A-Z]/.test(value)) testsPassed++; // has an uppercase letter
            if(/\d/.test(value)) testsPassed++; // has a digit
            if(/[^\dA-Za-z]/.test(value)) testsPassed++; // has a character that is not a letter or number
            return (testsPassed >= 3);
        });

        

        $(&quot; , &quot;'&quot; , &quot;.aliasOption&quot; , &quot;'&quot; , &quot;).click(function() {
            let inputField = $(&quot; , &quot;'&quot; , &quot;#alias input:text&quot; , &quot;'&quot; , &quot;)[0];
            $(inputField).val($(this).text());
        });

        $(&quot;.signup-form&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                email_confirm: {
                    required: true,
                    equalTo: &quot; , &quot;'&quot; , &quot;.signup-form [name=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;
                },
                alias: {
                    required: true,
                    minlength: 4,
                },
                password: {
                    required: true,
                    minlength: 8,
                    maxlength: 40,
                    pwstrength: true
                },
                password_confirm :{
                    required: true,
                    equalTo: &quot; , &quot;'&quot; , &quot;.signup-form [name=&quot;password&quot;]&quot; , &quot;'&quot; , &quot;,
                },
                accept_tos: {
                    required: true
                },
            },
            messages: {
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                },
                email_confirm: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;,
                    equalTo: &quot;Your confirmed email doesn&amp;#039;t match.&quot;
                },
                alias: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Invalid alias (only letters, numbers, periods, under-scores, and hyphens)&quot;,
                },
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;Weak Password:  Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    maxlength: &quot;Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                    pwstrength: &quot;Weak Password: Passwords must be between 8 and 40 characters long and must contain at least three of the following: 1 lowercase letter, 1 uppercase letter, 1 number, and 1 special character.&quot;,
                },
                password_confirm: {
                    required: &quot;This field is required.&quot;,
                    equalTo: &quot;Your confirmed password doesn&amp;#039;t match.&quot;,
                },
                accept_tos: {
                    required: &quot;This field is required.&quot;,
                },
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;)){
                    return;
                }
                $(form).addClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;.uses-recaptcha&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken(&quot; , &quot;'&quot; , &quot;signup&quot; , &quot;'&quot; , &quot;);

                form.submit();
            }
        });

        $(&quot; , &quot;'&quot; , &quot;.signupPlat.pc&quot; , &quot;'&quot; , &quot;).click(function(e){
            e.preventDefault();

            dataLayer.push({
                &quot;event&quot;: &quot;account-signup&quot;,
                &quot;eventCategory&quot;: &quot;account-signup&quot;,
                &quot;eventAction&quot;: &quot;open-lightbox&quot;,
            });

            $(&quot; , &quot;'&quot; , &quot;#signupPlatforms,.whatWill&quot; , &quot;'&quot; , &quot;).fadeOut(200,function(){
                $(&quot; , &quot;'&quot; , &quot;.pcSignup&quot; , &quot;'&quot; , &quot;).fadeIn(200);
            });
        });

        
    });

                                    
            
        

        
            
                
                
                                            
    
        
            
                
                
                
                Log In With
            
            
                
                
                
            
            
                
                
                
                or
            
            
                                
                    Email Address:
                    
                

                
                    Password:
                    
                
                
                    Remember Me
                
                Forgot Password
                
                
                    
                        
                            
                        
                    
                
                
                
                
    
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

    

    

            
            
                
                
                
                or
            
            
                
                    Sign Up
                
            
        
    


    $(document).ready(function () {
        $(&quot;form#loginForm&quot;).validate({
            rules: {
                email: {
                    required: true,
                    email: true
                },
                password: {
                    required: true,
                    minlength: 4
                },
            },
            messages: {
                password: {
                    required: &quot;This field is required.&quot;,
                    minlength: &quot;You must enter a valid password.&quot;
                },
                email: {
                    required: &quot;This field is required.&quot;,
                    email: &quot;Invalid Email Address&quot;
                }
            },
            errorPlacement: function(error, element) {
                error.appendTo( element.parent() );
            },
            submitHandler: async function(form){
                if($(form).hasClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;)){
                    return;
                }
                $(form).addClass(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;.uses-recaptcha&quot; , &quot;'&quot; , &quot;).prop(&quot;disabled&quot;, true);

                await fetchRecaptchaToken(&quot; , &quot;'&quot; , &quot;login&quot; , &quot;'&quot; , &quot;);

                form.submit();
            }
        });
    });

                                    
            
        

&quot;))]</value>
      <webElementGuid>2fcdec32-a519-427a-84d1-4754bbd920b4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
