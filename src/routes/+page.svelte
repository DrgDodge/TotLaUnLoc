<div class="welcome-container">
  <img src="/icons/lock.png" alt="Lock icon" class="welcome-icon" />
  <h1>Welcome to TotLaUnLoc</h1>
  <p>A really smart password manager and helper</p>
  <a href="/passwords" class="get-started-btn">Get Started</a>
</div>

<style>
  .welcome-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
  }

  .welcome-icon {
    width: 4rem;
    height: 4rem;
    margin-bottom: 1rem;
    filter: brightness(0) invert(1);
  }

  h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  p {
    font-size: 1.25rem;
    margin-bottom: 2rem;
  }

  .get-started-btn {
    background: #3a3a3a;
    color: #e0e0e0;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    text-decoration: none;
    font-size: 1rem;
    font-weight: 600;
    transition: background 0.2s;
  }

  .get-started-btn:hover {
    background: #4a4a4a;
  }
</style>