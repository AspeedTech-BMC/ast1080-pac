#[doc = "Register `AT01C` reader"]
pub type R = crate::R<At01cSpec>;
#[doc = "Register `AT01C` writer"]
pub type W = crate::W<At01cSpec>;
#[doc = "Field `ATCAMCHRERFLG` reader - AT_CAM_CHR_ERFLG"]
pub type AtcamchrerflgR = crate::BitReader;
#[doc = "Field `ATCAMCHRLTFLG` reader - AT_CAM_CHR_LTFLG"]
pub type AtcamchrltflgR = crate::BitReader;
#[doc = "Field `ATCAMCHFERFLG` reader - AT_CAM_CHF_ERFLG"]
pub type AtcamchferflgR = crate::BitReader;
#[doc = "Field `ATCAMCHFLTFLG` reader - AT_CAM_CHF_LTFLG"]
pub type AtcamchfltflgR = crate::BitReader;
#[doc = "Field `ATCAMCHRCOUNT` reader - AT_CAM_CHR_COUNT"]
pub type AtcamchrcountR = crate::FieldReader;
#[doc = "Field `ATCAMCHFCOUNT` reader - AT_CAM_CHF_COUNT"]
pub type AtcamchfcountR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AT_CAM_CHR_ERFLG"]
    #[inline(always)]
    pub fn atcamchrerflg(&self) -> AtcamchrerflgR {
        AtcamchrerflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_CHR_LTFLG"]
    #[inline(always)]
    pub fn atcamchrltflg(&self) -> AtcamchrltflgR {
        AtcamchrltflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_CAM_CHF_ERFLG"]
    #[inline(always)]
    pub fn atcamchferflg(&self) -> AtcamchferflgR {
        AtcamchferflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AT_CAM_CHF_LTFLG"]
    #[inline(always)]
    pub fn atcamchfltflg(&self) -> AtcamchfltflgR {
        AtcamchfltflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - AT_CAM_CHR_COUNT"]
    #[inline(always)]
    pub fn atcamchrcount(&self) -> AtcamchrcountR {
        AtcamchrcountR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - AT_CAM_CHF_COUNT"]
    #[inline(always)]
    pub fn atcamchfcount(&self) -> AtcamchfcountR {
        AtcamchfcountR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Clock Attack Monitor Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`at01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At01cSpec;
impl crate::RegisterSpec for At01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at01c::R`](R) reader structure"]
impl crate::Readable for At01cSpec {}
#[doc = "`write(|w| ..)` method takes [`at01c::W`](W) writer structure"]
impl crate::Writable for At01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT01C to value 0"]
impl crate::Resettable for At01cSpec {}
