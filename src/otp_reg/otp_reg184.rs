#[doc = "Register `OTP_REG184` reader"]
pub type R = crate::R<OtpReg184Spec>;
#[doc = "Register `OTP_REG184` writer"]
pub type W = crate::W<OtpReg184Spec>;
#[doc = "Field `REGRBPSOCKEYRETIREREN` reader - REG_RBP_SOC_KEYRETIRE_REN"]
pub type RegrbpsockeyretirerenR = crate::FieldReader;
#[doc = "Field `REGRBPSOCKEYRETIREREN` writer - REG_RBP_SOC_KEYRETIRE_REN"]
pub type RegrbpsockeyretirerenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPSOCKEYRETIREWEN` reader - REG_RBP_SOC_KEYRETIRE_WEN"]
pub type RegrbpsockeyretirewenR = crate::FieldReader;
#[doc = "Field `REGRBPSOCKEYRETIREWEN` writer - REG_RBP_SOC_KEYRETIRE_WEN"]
pub type RegrbpsockeyretirewenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPSOCKEYRETIRELOCK` reader - REG_RBP_SOC_KEYRETIRE_LOCK"]
pub type RegrbpsockeyretirelockR = crate::BitReader;
#[doc = "Field `REGRBPSOCKEYRETIRELOCK` writer - REG_RBP_SOC_KEYRETIRE_LOCK"]
pub type RegrbpsockeyretirelockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_RBP_SOC_KEYRETIRE_REN"]
    #[inline(always)]
    pub fn regrbpsockeyretireren(&self) -> RegrbpsockeyretirerenR {
        RegrbpsockeyretirerenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RBP_SOC_KEYRETIRE_WEN"]
    #[inline(always)]
    pub fn regrbpsockeyretirewen(&self) -> RegrbpsockeyretirewenR {
        RegrbpsockeyretirewenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_RBP_SOC_KEYRETIRE_LOCK"]
    #[inline(always)]
    pub fn regrbpsockeyretirelock(&self) -> RegrbpsockeyretirelockR {
        RegrbpsockeyretirelockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RBP_SOC_KEYRETIRE_REN"]
    #[inline(always)]
    pub fn regrbpsockeyretireren(&mut self) -> RegrbpsockeyretirerenW<OtpReg184Spec> {
        RegrbpsockeyretirerenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RBP_SOC_KEYRETIRE_WEN"]
    #[inline(always)]
    pub fn regrbpsockeyretirewen(&mut self) -> RegrbpsockeyretirewenW<OtpReg184Spec> {
        RegrbpsockeyretirewenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_RBP_SOC_KEYRETIRE_LOCK"]
    #[inline(always)]
    pub fn regrbpsockeyretirelock(&mut self) -> RegrbpsockeyretirelockW<OtpReg184Spec> {
        RegrbpsockeyretirelockW::new(self, 31)
    }
}
#[doc = "OTP\\_RBP\\_SOC\\_KEYRETIRE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg184::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg184::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg184Spec;
impl crate::RegisterSpec for OtpReg184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg184::R`](R) reader structure"]
impl crate::Readable for OtpReg184Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg184::W`](W) writer structure"]
impl crate::Writable for OtpReg184Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG184 to value 0xffff"]
impl crate::Resettable for OtpReg184Spec {
    const RESET_VALUE: u32 = 0xffff;
}
