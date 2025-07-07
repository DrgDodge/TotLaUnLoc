<script lang="ts">
  import { t } from '../language';
</script>

<div class="welcome-container">
  <img src="/icons/Logoo.png" alt="Welcome icon" class="welcome-icon" />
  <h1>{$t('welcome_title')}</h1>
  <p>{$t('welcome_subtitle')}</p>
  <a href="/passwords" class="get-started-btn">{$t('get_started')}</a>
</div>

<style>
  .welcome-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    text-align: center;
    margin: auto
  }

  .welcome-icon {
    width: 20rem;
    height: 20rem;
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