#[doc = "Register `SCU3B0` reader"]
pub type R = crate::R<Scu3b0Spec>;
#[doc = "Register `SCU3B0` writer"]
pub type W = crate::W<Scu3b0Spec>;
#[doc = "Field `SCUUSBCSEL` reader - SCU_USBC_SEL"]
pub type ScuusbcselR = crate::FieldReader;
#[doc = "Field `SCUUSBCSEL` writer - SCU_USBC_SEL"]
pub type ScuusbcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUUSBDSEL` reader - SCU_USBD_SEL"]
pub type ScuusbdselR = crate::FieldReader;
#[doc = "Field `SCUUSBDSEL` writer - SCU_USBD_SEL"]
pub type ScuusbdselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SCU_USBC_SEL"]
    #[inline(always)]
    pub fn scuusbcsel(&self) -> ScuusbcselR {
        ScuusbcselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SCU_USBD_SEL"]
    #[inline(always)]
    pub fn scuusbdsel(&self) -> ScuusbdselR {
        ScuusbdselR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCU_USBC_SEL"]
    #[inline(always)]
    pub fn scuusbcsel(&mut self) -> ScuusbcselW<Scu3b0Spec> {
        ScuusbcselW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SCU_USBD_SEL"]
    #[inline(always)]
    pub fn scuusbdsel(&mut self) -> ScuusbdselW<Scu3b0Spec> {
        ScuusbdselW::new(self, 2)
    }
}
#[doc = "USB Controler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3b0Spec;
impl crate::RegisterSpec for Scu3b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3b0::R`](R) reader structure"]
impl crate::Readable for Scu3b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3b0::W`](W) writer structure"]
impl crate::Writable for Scu3b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3B0 to value 0x0a"]
impl crate::Resettable for Scu3b0Spec {
    const RESET_VALUE: u32 = 0x0a;
}
