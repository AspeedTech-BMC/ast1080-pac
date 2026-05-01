#[doc = "Register `VIC500` reader"]
pub type R = crate::R<Vic500Spec>;
#[doc = "Register `VIC500` writer"]
pub type W = crate::W<Vic500Spec>;
#[doc = "Field `VICRSTINTCSEL` reader - VIC_RST_INTC_SEL"]
pub type VicrstintcselR = crate::FieldReader;
#[doc = "Field `VICRSTINTCSEL` writer - VIC_RST_INTC_SEL"]
pub type VicrstintcselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - VIC_RST_INTC_SEL"]
    #[inline(always)]
    pub fn vicrstintcsel(&self) -> VicrstintcselR {
        VicrstintcselR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VIC_RST_INTC_SEL"]
    #[inline(always)]
    pub fn vicrstintcsel(&mut self) -> VicrstintcselW<Vic500Spec> {
        VicrstintcselW::new(self, 0)
    }
}
#[doc = "Reset INTC Select\n\nYou can [`read`](crate::Reg::read) this register and get [`vic500::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic500::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic500Spec;
impl crate::RegisterSpec for Vic500Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic500::R`](R) reader structure"]
impl crate::Readable for Vic500Spec {}
#[doc = "`write(|w| ..)` method takes [`vic500::W`](W) writer structure"]
impl crate::Writable for Vic500Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC500 to value 0"]
impl crate::Resettable for Vic500Spec {}
