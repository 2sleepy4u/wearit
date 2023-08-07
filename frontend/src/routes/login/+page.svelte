<div class="center">
    <Textfield
	type="email"
 bind:invalid={isValidEmail}
 updateInvalid
 bind:value={email}
 label="Email"
 style="min-width: 250px;"
 >
 <HelperText 
     style="display: block!important;"
     validationMsg slot="helper">
      That's not a valid email address.
    </HelperText>
    </Textfield>
    <Textfield
	type="password"
 bind:value={password}
 label="Password"
 style="min-width: 250px;"
 >
 <HelperText validationMsg slot="helper">
      That's not a valid user.
    </HelperText>
    </Textfield>
    <Button on:click={login}>
	<Label>Log In</Label>
    </Button>
</div>

<script lang="ts">
    import Textfield from '@smui/textfield';
    import HelperText from '@smui/textfield/helper-text';
    import Button, { Label } from "@smui/button"
    const LOGIN_API = "auth/new_session"
    let email: string = ""
    let password: string = ""


    let isValidEmail = false;

    async function login(){
	await fetch(LOGIN_API)
    }

</script>

<style>
    .center {
	margin: auto;
	text-align: center;
	margin-top: 150px;
    }
</style>
