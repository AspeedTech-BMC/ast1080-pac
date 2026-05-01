#[doc = "Register `SCU334` reader"]
pub type R = crate::R<Scu334Spec>;
#[doc = "Register `SCU334` writer"]
pub type W = crate::W<Scu334Spec>;
#[doc = "Field `SCUHUXR` reader - SCU_HUX_R"]
pub type ScuhuxrR = crate::FieldReader;
#[doc = "Field `SCUHUXR` writer - SCU_HUX_R"]
pub type ScuhuxrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCUHUXN` reader - SCU_HUX_N"]
pub type ScuhuxnR = crate::FieldReader<u16>;
#[doc = "Field `SCUHUXN` writer - SCU_HUX_N"]
pub type ScuhuxnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - SCU_HUX_R"]
    #[inline(always)]
    pub fn scuhuxr(&self) -> ScuhuxrR {
        ScuhuxrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - SCU_HUX_N"]
    #[inline(always)]
    pub fn scuhuxn(&self) -> ScuhuxnR {
        ScuhuxnR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCU_HUX_R"]
    #[inline(always)]
    pub fn scuhuxr(&mut self) -> ScuhuxrW<Scu334Spec> {
        ScuhuxrW::new(self, 0)
    }
    #[doc = "Bits 8:17 - SCU_HUX_N"]
    #[inline(always)]
    pub fn scuhuxn(&mut self) -> ScuhuxnW<Scu334Spec> {
        ScuhuxnW::new(self, 8)
    }
}
#[doc = "HUARTCLK Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu334::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu334::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu334Spec;
impl crate::RegisterSpec for Scu334Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu334::R`](R) reader structure"]
impl crate::Readable for Scu334Spec {}
#[doc = "`write(|w| ..)` method takes [`scu334::W`](W) writer structure"]
impl crate::Writable for Scu334Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU334 to value 0x0001_4560"]
impl crate::Resettable for Scu334Spec {
    const RESET_VALUE: u32 = 0x0001_4560;
}
