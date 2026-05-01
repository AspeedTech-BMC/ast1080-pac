#[doc = "Register `OTP_REG0D0` reader"]
pub type R = crate::R<OtpReg0d0Spec>;
#[doc = "Register `OTP_REG0D0` writer"]
pub type W = crate::W<OtpReg0d0Spec>;
#[doc = "Field `PID` reader - PID"]
pub type PidR = crate::FieldReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PSECURE` reader - PSECURE"]
pub type PsecureR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGPUFSEL` reader - REG_PUF_SEL"]
pub type RegpufselR = crate::FieldReader;
#[doc = "Field `REGPUFSEL` writer - REG_PUF_SEL"]
pub type RegpufselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - PSECURE"]
    #[inline(always)]
    pub fn psecure(&self) -> PsecureR {
        PsecureR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - REG_PUF_SEL"]
    #[inline(always)]
    pub fn regpufsel(&self) -> RegpufselR {
        RegpufselR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - REG_PUF_SEL"]
    #[inline(always)]
    pub fn regpufsel(&mut self) -> RegpufselW<OtpReg0d0Spec> {
        RegpufselW::new(self, 12)
    }
}
#[doc = "pid\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0d0Spec;
impl crate::RegisterSpec for OtpReg0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0d0::R`](R) reader structure"]
impl crate::Readable for OtpReg0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0d0::W`](W) writer structure"]
impl crate::Writable for OtpReg0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0D0 to value 0"]
impl crate::Resettable for OtpReg0d0Spec {}
