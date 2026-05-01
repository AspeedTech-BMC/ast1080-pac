#[doc = "Register `AT024` reader"]
pub type R = crate::R<At024Spec>;
#[doc = "Register `AT024` writer"]
pub type W = crate::W<At024Spec>;
#[doc = "Field `ATCAMCHRINTSTS` reader - AT_CAM_CHR_INT_STS"]
pub type AtcamchrintstsR = crate::BitReader;
#[doc = "Field `ATCAMCHRINTSTS` writer - AT_CAM_CHR_INT_STS"]
pub type AtcamchrintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMCHFINTSTS` reader - AT_CAM_CHF_INT_STS"]
pub type AtcamchfintstsR = crate::BitReader;
#[doc = "Field `ATCAMCHFINTSTS` writer - AT_CAM_CHF_INT_STS"]
pub type AtcamchfintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMGDINTSTS` reader - AT_CAM_GD_INT_STS"]
pub type AtcamgdintstsR = crate::BitReader;
#[doc = "Field `ATCAMGDINTSTS` writer - AT_CAM_GD_INT_STS"]
pub type AtcamgdintstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_CAM_CHR_INT_STS"]
    #[inline(always)]
    pub fn atcamchrintsts(&self) -> AtcamchrintstsR {
        AtcamchrintstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_INT_STS"]
    #[inline(always)]
    pub fn atcamchfintsts(&self) -> AtcamchfintstsR {
        AtcamchfintstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_CAM_GD_INT_STS"]
    #[inline(always)]
    pub fn atcamgdintsts(&self) -> AtcamgdintstsR {
        AtcamgdintstsR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_CAM_CHR_INT_STS"]
    #[inline(always)]
    pub fn atcamchrintsts(&mut self) -> AtcamchrintstsW<At024Spec> {
        AtcamchrintstsW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHF_INT_STS"]
    #[inline(always)]
    pub fn atcamchfintsts(&mut self) -> AtcamchfintstsW<At024Spec> {
        AtcamchfintstsW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_CAM_GD_INT_STS"]
    #[inline(always)]
    pub fn atcamgdintsts(&mut self) -> AtcamgdintstsW<At024Spec> {
        AtcamgdintstsW::new(self, 2)
    }
}
#[doc = "Clock Attack Monitor Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At024Spec;
impl crate::RegisterSpec for At024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at024::R`](R) reader structure"]
impl crate::Readable for At024Spec {}
#[doc = "`write(|w| ..)` method takes [`at024::W`](W) writer structure"]
impl crate::Writable for At024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT024 to value 0"]
impl crate::Resettable for At024Spec {}
