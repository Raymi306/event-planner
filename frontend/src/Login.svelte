<script>
    let email;
    let password;
    let totp;
</script>

<h1>Login</h1>
<form>
  <input
    class="form-field"
    bind:value="{email}"
    type="email"
    placeholder="Email"
  />
  <input
    class="form-field"
    bind:value="{password}"
    type="password"
    placeholder="Password"
  />
  <input
    class="form-field"
    bind:value="{totp}"
    type="text"
    placeholder="TOTP"
  />

  <button class="form-field">
    Login
  </button>
</form>
