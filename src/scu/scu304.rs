#[doc = "Register `SCU304` reader"]
pub type R = crate::R<Scu304Spec>;
#[doc = "Register `SCU304` writer"]
pub type W = crate::W<Scu304Spec>;
#[doc = "Field `SCUHPLLBWADJ` reader - SCU_HPLL_BWADJ"]
pub type ScuhpllbwadjR = crate::FieldReader<u16>;
#[doc = "Field `SCUHPLLBWADJ` writer - SCU_HPLL_BWADJ"]
pub type ScuhpllbwadjW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCUHPLLLOCK` reader - SCU_HPLL_LOCK"]
pub type ScuhplllockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - SCU_HPLL_BWADJ"]
    #[inline(always)]
    pub fn scuhpllbwadj(&self) -> ScuhpllbwadjR {
        ScuhpllbwadjR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - SCU_HPLL_LOCK"]
    #[inline(always)]
    pub fn scuhplllock(&self) -> ScuhplllockR {
        ScuhplllockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - SCU_HPLL_BWADJ"]
    #[inline(always)]
    pub fn scuhpllbwadj(&mut self) -> ScuhpllbwadjW<Scu304Spec> {
        ScuhpllbwadjW::new(self, 0)
    }
}
#[doc = "HPLL Extended Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu304::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu304::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu304Spec;
impl crate::RegisterSpec for Scu304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu304::R`](R) reader structure"]
impl crate::Readable for Scu304Spec {}
#[doc = "`write(|w| ..)` method takes [`scu304::W`](W) writer structure"]
impl crate::Writable for Scu304Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU304 to value 0x27"]
impl crate::Resettable for Scu304Spec {
    const RESET_VALUE: u32 = 0x27;
}
