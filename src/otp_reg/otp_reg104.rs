#[doc = "Register `OTP_REG104` reader"]
pub type R = crate::R<OtpReg104Spec>;
#[doc = "Register `OTP_REG104` writer"]
pub type W = crate::W<OtpReg104Spec>;
#[doc = "Field `REGREGIONOTPCFGREN` reader - REG_REGION_OTPCFG_REN"]
pub type RegregionotpcfgrenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPCFGREN` writer - REG_REGION_OTPCFG_REN"]
pub type RegregionotpcfgrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPCFGWEN` reader - REG_REGION_OTPCFG_WEN"]
pub type RegregionotpcfgwenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPCFGWEN` writer - REG_REGION_OTPCFG_WEN"]
pub type RegregionotpcfgwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPCFGLOCK` reader - REG_REGION_OTPCFG_LOCK"]
pub type RegregionotpcfglockR = crate::BitReader;
#[doc = "Field `REGREGIONOTPCFGLOCK` writer - REG_REGION_OTPCFG_LOCK"]
pub type RegregionotpcfglockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_OTPCFG_REN"]
    #[inline(always)]
    pub fn regregionotpcfgren(&self) -> RegregionotpcfgrenR {
        RegregionotpcfgrenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTPCFG_WEN"]
    #[inline(always)]
    pub fn regregionotpcfgwen(&self) -> RegregionotpcfgwenR {
        RegregionotpcfgwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_OTPCFG_LOCK"]
    #[inline(always)]
    pub fn regregionotpcfglock(&self) -> RegregionotpcfglockR {
        RegregionotpcfglockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_OTPCFG_REN"]
    #[inline(always)]
    pub fn regregionotpcfgren(&mut self) -> RegregionotpcfgrenW<OtpReg104Spec> {
        RegregionotpcfgrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTPCFG_WEN"]
    #[inline(always)]
    pub fn regregionotpcfgwen(&mut self) -> RegregionotpcfgwenW<OtpReg104Spec> {
        RegregionotpcfgwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_OTPCFG_LOCK"]
    #[inline(always)]
    pub fn regregionotpcfglock(&mut self) -> RegregionotpcfglockW<OtpReg104Spec> {
        RegregionotpcfglockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_OTPCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg104Spec;
impl crate::RegisterSpec for OtpReg104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg104::R`](R) reader structure"]
impl crate::Readable for OtpReg104Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg104::W`](W) writer structure"]
impl crate::Writable for OtpReg104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG104 to value 0xffff"]
impl crate::Resettable for OtpReg104Spec {
    const RESET_VALUE: u32 = 0xffff;
}
