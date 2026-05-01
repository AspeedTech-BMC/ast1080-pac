#[doc = "Register `SCU104` reader"]
pub type R = crate::R<Scu104Spec>;
#[doc = "Register `SCU104` writer"]
pub type W = crate::W<Scu104Spec>;
#[doc = "Field `SCULPCRSTFALLINGEN` reader - SCU_LPC_RST_FALLING_EN"]
pub type SculpcrstfallingenR = crate::BitReader;
#[doc = "Field `SCULPCRSTFALLINGEN` writer - SCU_LPC_RST_FALLING_EN"]
pub type SculpcrstfallingenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCULPCRSTRISINGEN` reader - SCU_LPC_RST_RISING_EN"]
pub type SculpcrstrisingenR = crate::BitReader;
#[doc = "Field `SCULPCRSTRISINGEN` writer - SCU_LPC_RST_RISING_EN"]
pub type SculpcrstrisingenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_LPC_RST_FALLING_EN"]
    #[inline(always)]
    pub fn sculpcrstfallingen(&self) -> SculpcrstfallingenR {
        SculpcrstfallingenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_LPC_RST_RISING_EN"]
    #[inline(always)]
    pub fn sculpcrstrisingen(&self) -> SculpcrstrisingenR {
        SculpcrstrisingenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_LPC_RST_FALLING_EN"]
    #[inline(always)]
    pub fn sculpcrstfallingen(&mut self) -> SculpcrstfallingenW<Scu104Spec> {
        SculpcrstfallingenW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_LPC_RST_RISING_EN"]
    #[inline(always)]
    pub fn sculpcrstrisingen(&mut self) -> SculpcrstrisingenW<Scu104Spec> {
        SculpcrstrisingenW::new(self, 1)
    }
}
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu104Spec;
impl crate::RegisterSpec for Scu104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu104::R`](R) reader structure"]
impl crate::Readable for Scu104Spec {}
#[doc = "`write(|w| ..)` method takes [`scu104::W`](W) writer structure"]
impl crate::Writable for Scu104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU104 to value 0"]
impl crate::Resettable for Scu104Spec {}
