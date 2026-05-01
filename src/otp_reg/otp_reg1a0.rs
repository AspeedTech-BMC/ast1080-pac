#[doc = "Register `OTP_REG1A0` reader"]
pub type R = crate::R<OtpReg1a0Spec>;
#[doc = "Register `OTP_REG1A0` writer"]
pub type W = crate::W<OtpReg1a0Spec>;
#[doc = "Field `REGSWPUFWLOCK` reader - REG_SW_PUF_WLOCK"]
pub type RegswpufwlockR = crate::BitReader;
#[doc = "Field `REGSWPUFWLOCK` writer - REG_SW_PUF_WLOCK"]
pub type RegswpufwlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHWPUFWLOCK` reader - REG_HW_PUF_WLOCK"]
pub type ReghwpufwlockR = crate::BitReader;
#[doc = "Field `REGHWPUFWLOCK` writer - REG_HW_PUF_WLOCK"]
pub type ReghwpufwlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSWPUFRLOCK` reader - REG_SW_PUF_RLOCK"]
pub type RegswpufrlockR = crate::BitReader;
#[doc = "Field `REGSWPUFRLOCK` writer - REG_SW_PUF_RLOCK"]
pub type RegswpufrlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_SW_PUF_WLOCK"]
    #[inline(always)]
    pub fn regswpufwlock(&self) -> RegswpufwlockR {
        RegswpufwlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_HW_PUF_WLOCK"]
    #[inline(always)]
    pub fn reghwpufwlock(&self) -> ReghwpufwlockR {
        ReghwpufwlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - REG_SW_PUF_RLOCK"]
    #[inline(always)]
    pub fn regswpufrlock(&self) -> RegswpufrlockR {
        RegswpufrlockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_SW_PUF_WLOCK"]
    #[inline(always)]
    pub fn regswpufwlock(&mut self) -> RegswpufwlockW<OtpReg1a0Spec> {
        RegswpufwlockW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_HW_PUF_WLOCK"]
    #[inline(always)]
    pub fn reghwpufwlock(&mut self) -> ReghwpufwlockW<OtpReg1a0Spec> {
        ReghwpufwlockW::new(self, 1)
    }
    #[doc = "Bit 8 - REG_SW_PUF_RLOCK"]
    #[inline(always)]
    pub fn regswpufrlock(&mut self) -> RegswpufrlockW<OtpReg1a0Spec> {
        RegswpufrlockW::new(self, 8)
    }
}
#[doc = "PUF\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1a0Spec;
impl crate::RegisterSpec for OtpReg1a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1a0::R`](R) reader structure"]
impl crate::Readable for OtpReg1a0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1a0::W`](W) writer structure"]
impl crate::Writable for OtpReg1a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1A0 to value 0"]
impl crate::Resettable for OtpReg1a0Spec {}
