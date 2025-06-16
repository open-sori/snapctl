import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/quickstart">
            Quick Start
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title}`}
      description="Description will go into a meta tag in <head />">
      <HomepageHeader />
      <main>
        <section className={styles.features}>
          <div className="container">
            <div className="row">
              <div className="col col--12">
                <img src="https://raw.githubusercontent.com/open-sori/logos/main/logo-full-color.svg" alt="Open-Sori Logo" style={{ maxWidth: '400px', marginBottom: '20px', display: 'block', marginLeft: 'auto', marginRight: 'auto', marginTop: '40px' }} />
                <p style={{ textAlign: 'center', marginBottom: '20px' }}>
                  Snaptui is a powerful terminal user interface (TUI) application written in Rust, designed for efficient management of Mumble servers. It provides a real-time overview of clients, groups, and streams, allowing administrators to easily modify user properties, manage group settings, and monitor server activity directly from the terminal. Built with a focus on performance and responsiveness, Snaptui offers a streamlined experience for Mumble server administration.
                </p>
                <div style={{ display: 'flex', justifyContent: 'center', marginBottom: '20px' }}>
                  <pre><code>snapctl --host 127.0.0.1 --port 1780 get clients</code></pre>
                </div>
                <div style={{ display: 'flex', justifyContent: 'center', marginBottom: '40px' }}>
                  <video controls width="1440" height="460">
                    <source src="/video/intro.webm" type="video/webm" />
                    Your browser does not support the video tag.
                  </video>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
