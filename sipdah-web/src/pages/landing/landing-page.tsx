import { About } from '@/pages/landing/components/about'
import { Cta } from '@/pages/landing/components/cta'
import { FAQ } from '@/pages/landing/components/faq'
import { Features } from '@/pages/landing/components/features'
import { Footer } from '@/pages/landing/components/footer'
import { Hero } from '@/pages/landing/components/hero'
import { HowItWorks } from '@/pages/landing/components/how-it-works'
import { Navbar } from '@/pages/landing/components/navbar'
import { Newsletter } from '@/pages/landing/components/news-letter'
import { Pricing } from '@/pages/landing/components/pricing'
import { ScrollToTop } from '@/pages/landing/components/scroll-to-top'
import { Services } from '@/pages/landing/components/services'
import { Sponsors } from '@/pages/landing/components/sponsors'
import { Team } from '@/pages/landing/components/team'
import { Testimonials } from '@/pages/landing/components/testimonials'
import { rootRoute } from '@/router-config'
import { createRoute, HeadContent } from '@tanstack/react-router'

export const landingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => (
    <>
      <HeadContent />
      <LandingPage />
    </>
  ),
  head() {
    return {
      meta: [{ title: 'Sipdah' }],
    }
  },
})

export function LandingPage() {
  return (
    <main className='w-full flex flex-col items-center'>
      <Navbar />
      <Hero />
      <Sponsors />
      <About />
      <HowItWorks />
      <Features />
      <Services />
      <Cta />
      <Testimonials />
      <Team />
      <Pricing />
      <Newsletter />
      <FAQ />
      <Footer />
      <ScrollToTop />
    </main>
  )
}
