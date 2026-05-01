#[doc = "Register `AT020` reader"]
pub type R = crate::R<At020Spec>;
#[doc = "Register `AT020` writer"]
pub type W = crate::W<At020Spec>;
#[doc = "Field `ATCAMFQCVALID` reader - AT_CAM_FQC_VALID"]
pub type AtcamfqcvalidR = crate::BitReader;
#[doc = "Field `ATCAMFQCREFVAL` reader - AT_CAM_FQC_REFVAL"]
pub type AtcamfqcrefvalR = crate::FieldReader<u16>;
#[doc = "Field `ATCAMFQCCMVAL` reader - AT_CAM_FQC_CMVAL"]
pub type AtcamfqccmvalR = crate::FieldReader;
#[doc = "Field `ATCAMGDPFLG` reader - AT_CAM_GD_PFLG"]
pub type AtcamgdpflgR = crate::BitReader;
#[doc = "Field `ATCAMGDNFLG` reader - AT_CAM_GD_NFLG"]
pub type AtcamgdnflgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AT_CAM_FQC_VALID"]
    #[inline(always)]
    pub fn atcamfqcvalid(&self) -> AtcamfqcvalidR {
        AtcamfqcvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:12 - AT_CAM_FQC_REFVAL"]
    #[inline(always)]
    pub fn atcamfqcrefval(&self) -> AtcamfqcrefvalR {
        AtcamfqcrefvalR::new(((self.bits >> 1) & 0x0fff) as u16)
    }
    #[doc = "Bits 13:18 - AT_CAM_FQC_CMVAL"]
    #[inline(always)]
    pub fn atcamfqccmval(&self) -> AtcamfqccmvalR {
        AtcamfqccmvalR::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bit 19 - AT_CAM_GD_PFLG"]
    #[inline(always)]
    pub fn atcamgdpflg(&self) -> AtcamgdpflgR {
        AtcamgdpflgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AT_CAM_GD_NFLG"]
    #[inline(always)]
    pub fn atcamgdnflg(&self) -> AtcamgdnflgR {
        AtcamgdnflgR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {}
#[doc = "Clock Attack Monitor Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`at020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At020Spec;
impl crate::RegisterSpec for At020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at020::R`](R) reader structure"]
impl crate::Readable for At020Spec {}
#[doc = "`write(|w| ..)` method takes [`at020::W`](W) writer structure"]
impl crate::Writable for At020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT020 to value 0"]
impl crate::Resettable for At020Spec {}
