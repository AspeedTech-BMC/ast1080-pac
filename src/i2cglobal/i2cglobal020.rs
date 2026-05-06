#[doc = "Register `I2CGLOBAL020` reader"]
pub type R = crate::R<I2cglobal020Spec>;
#[doc = "Register `I2CGLOBAL020` writer"]
pub type W = crate::W<I2cglobal020Spec>;
#[doc = "Field `GWLOCK` reader - GWLOCK"]
pub type GwlockR = crate::FieldReader;
#[doc = "Field `GWLOCK` writer - GWLOCK"]
pub type GwlockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GWLOCK"]
    #[inline(always)]
    pub fn gwlock(&self) -> GwlockR {
        GwlockR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - GWLOCK"]
    #[inline(always)]
    pub fn gwlock(&mut self) -> GwlockW<I2cglobal020Spec> {
        GwlockW::new(self, 0)
    }
}
#[doc = "Write lock protection Register for Security\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cglobal020Spec;
impl crate::RegisterSpec for I2cglobal020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cglobal020::R`](R) reader structure"]
impl crate::Readable for I2cglobal020Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cglobal020::W`](W) writer structure"]
impl crate::Writable for I2cglobal020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CGLOBAL020 to value 0"]
impl crate::Resettable for I2cglobal020Spec {}
