#[doc = "Register `SPIF300` reader"]
pub type R = crate::R<Spif300Spec>;
#[doc = "Register `SPIF300` writer"]
pub type W = crate::W<Spif300Spec>;
#[doc = "Field `WPWTABLEEN` reader - WP_WTABLE_EN"]
pub type WpwtableenR = crate::FieldReader<u32>;
#[doc = "Field `WPWTABLEEN` writer - WP_WTABLE_EN"]
pub type WpwtableenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WP_WTABLE_EN"]
    #[inline(always)]
    pub fn wpwtableen(&self) -> WpwtableenR {
        WpwtableenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WP_WTABLE_EN"]
    #[inline(always)]
    pub fn wpwtableen(&mut self) -> WpwtableenW<Spif300Spec> {
        WpwtableenW::new(self, 0)
    }
}
#[doc = "SPIF\\_WPWTABLE\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`spif300::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif300::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif300Spec;
impl crate::RegisterSpec for Spif300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif300::R`](R) reader structure"]
impl crate::Readable for Spif300Spec {}
#[doc = "`write(|w| ..)` method takes [`spif300::W`](W) writer structure"]
impl crate::Writable for Spif300Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF300 to value 0"]
impl crate::Resettable for Spif300Spec {}
