#[doc = "Register `SCU330` reader"]
pub type R = crate::R<Scu330Spec>;
#[doc = "Register `SCU330` writer"]
pub type W = crate::W<Scu330Spec>;
#[doc = "Field `SCUUXR` reader - SCU_UX_R"]
pub type ScuuxrR = crate::FieldReader;
#[doc = "Field `SCUUXR` writer - SCU_UX_R"]
pub type ScuuxrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCUUXN` reader - SCU_UX_N"]
pub type ScuuxnR = crate::FieldReader<u16>;
#[doc = "Field `SCUUXN` writer - SCU_UX_N"]
pub type ScuuxnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - SCU_UX_R"]
    #[inline(always)]
    pub fn scuuxr(&self) -> ScuuxrR {
        ScuuxrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - SCU_UX_N"]
    #[inline(always)]
    pub fn scuuxn(&self) -> ScuuxnR {
        ScuuxnR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCU_UX_R"]
    #[inline(always)]
    pub fn scuuxr(&mut self) -> ScuuxrW<Scu330Spec> {
        ScuuxrW::new(self, 0)
    }
    #[doc = "Bits 8:17 - SCU_UX_N"]
    #[inline(always)]
    pub fn scuuxn(&mut self) -> ScuuxnW<Scu330Spec> {
        ScuuxnW::new(self, 8)
    }
}
#[doc = "UARTCLK Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu330::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu330::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu330Spec;
impl crate::RegisterSpec for Scu330Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu330::R`](R) reader structure"]
impl crate::Readable for Scu330Spec {}
#[doc = "`write(|w| ..)` method takes [`scu330::W`](W) writer structure"]
impl crate::Writable for Scu330Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU330 to value 0x0001_4503"]
impl crate::Resettable for Scu330Spec {
    const RESET_VALUE: u32 = 0x0001_4503;
}
