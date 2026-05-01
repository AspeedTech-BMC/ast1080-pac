#[doc = "Register `OTP_REG18C` reader"]
pub type R = crate::R<OtpReg18cSpec>;
#[doc = "Register `OTP_REG18C` writer"]
pub type W = crate::W<OtpReg18cSpec>;
#[doc = "Field `REGRBPCALIPTRAKEYRETIREREN` reader - REG_RBP_CALIPTRA_KEYRETIRE_REN"]
pub type RegrbpcaliptrakeyretirerenR = crate::FieldReader;
#[doc = "Field `REGRBPCALIPTRAKEYRETIREREN` writer - REG_RBP_CALIPTRA_KEYRETIRE_REN"]
pub type RegrbpcaliptrakeyretirerenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPCALIPTRAKEYRETIREWEN` reader - REG_RBP_CALIPTRA_KEYRETIRE_WEN"]
pub type RegrbpcaliptrakeyretirewenR = crate::FieldReader;
#[doc = "Field `REGRBPCALIPTRAKEYRETIREWEN` writer - REG_RBP_CALIPTRA_KEYRETIRE_WEN"]
pub type RegrbpcaliptrakeyretirewenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPCALIPTRAKEYRETIRELOCK` reader - REG_RBP_CALIPTRA_KEYRETIRE_LOCK"]
pub type RegrbpcaliptrakeyretirelockR = crate::BitReader;
#[doc = "Field `REGRBPCALIPTRAKEYRETIRELOCK` writer - REG_RBP_CALIPTRA_KEYRETIRE_LOCK"]
pub type RegrbpcaliptrakeyretirelockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_RBP_CALIPTRA_KEYRETIRE_REN"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretireren(&self) -> RegrbpcaliptrakeyretirerenR {
        RegrbpcaliptrakeyretirerenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RBP_CALIPTRA_KEYRETIRE_WEN"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretirewen(&self) -> RegrbpcaliptrakeyretirewenR {
        RegrbpcaliptrakeyretirewenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_RBP_CALIPTRA_KEYRETIRE_LOCK"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretirelock(&self) -> RegrbpcaliptrakeyretirelockR {
        RegrbpcaliptrakeyretirelockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RBP_CALIPTRA_KEYRETIRE_REN"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretireren(&mut self) -> RegrbpcaliptrakeyretirerenW<OtpReg18cSpec> {
        RegrbpcaliptrakeyretirerenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RBP_CALIPTRA_KEYRETIRE_WEN"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretirewen(&mut self) -> RegrbpcaliptrakeyretirewenW<OtpReg18cSpec> {
        RegrbpcaliptrakeyretirewenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_RBP_CALIPTRA_KEYRETIRE_LOCK"]
    #[inline(always)]
    pub fn regrbpcaliptrakeyretirelock(&mut self) -> RegrbpcaliptrakeyretirelockW<OtpReg18cSpec> {
        RegrbpcaliptrakeyretirelockW::new(self, 31)
    }
}
#[doc = "OTP\\_RBP\\_CALIP\\_KEYRETIRE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg18c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg18c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg18cSpec;
impl crate::RegisterSpec for OtpReg18cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg18c::R`](R) reader structure"]
impl crate::Readable for OtpReg18cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg18c::W`](W) writer structure"]
impl crate::Writable for OtpReg18cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG18C to value 0xffff"]
impl crate::Resettable for OtpReg18cSpec {
    const RESET_VALUE: u32 = 0xffff;
}
