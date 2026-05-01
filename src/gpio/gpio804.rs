#[doc = "Register `GPIO804` reader"]
pub type R = crate::R<Gpio804Spec>;
#[doc = "Register `GPIO804` writer"]
pub type W = crate::W<Gpio804Spec>;
#[doc = "Field `Master4ID` reader - Master #4 ID"]
pub type Master4idR = crate::FieldReader;
#[doc = "Field `Master4ID` writer - Master #4 ID"]
pub type Master4idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster4 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster4> for bool {
    #[inline(always)]
    fn from(variant: SecMaster4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster4` reader - Secure Master #4"]
pub type SecMaster4R = crate::BitReader<SecMaster4>;
impl SecMaster4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster4 {
        match self.bits {
            true => SecMaster4::Secure,
            false => SecMaster4::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster4::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster4::NonSecure
    }
}
#[doc = "Field `SecMaster4` writer - Secure Master #4"]
pub type SecMaster4W<'a, REG> = crate::BitWriter<'a, REG, SecMaster4>;
impl<'a, REG> SecMaster4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster4::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster4::NonSecure)
    }
}
#[doc = "Field `Master5ID` reader - Master #5 ID"]
pub type Master5idR = crate::FieldReader;
#[doc = "Field `Master5ID` writer - Master #5 ID"]
pub type Master5idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster5 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster5> for bool {
    #[inline(always)]
    fn from(variant: SecMaster5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster5` reader - Secure Master #5"]
pub type SecMaster5R = crate::BitReader<SecMaster5>;
impl SecMaster5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster5 {
        match self.bits {
            true => SecMaster5::Secure,
            false => SecMaster5::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster5::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster5::NonSecure
    }
}
#[doc = "Field `SecMaster5` writer - Secure Master #5"]
pub type SecMaster5W<'a, REG> = crate::BitWriter<'a, REG, SecMaster5>;
impl<'a, REG> SecMaster5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster5::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster5::NonSecure)
    }
}
#[doc = "Field `Master6ID` reader - Master #6 ID"]
pub type Master6idR = crate::FieldReader;
#[doc = "Field `Master6ID` writer - Master #6 ID"]
pub type Master6idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster6 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster6> for bool {
    #[inline(always)]
    fn from(variant: SecMaster6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster6` reader - Secure Master #6"]
pub type SecMaster6R = crate::BitReader<SecMaster6>;
impl SecMaster6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster6 {
        match self.bits {
            true => SecMaster6::Secure,
            false => SecMaster6::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster6::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster6::NonSecure
    }
}
#[doc = "Field `SecMaster6` writer - Secure Master #6"]
pub type SecMaster6W<'a, REG> = crate::BitWriter<'a, REG, SecMaster6>;
impl<'a, REG> SecMaster6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster6::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster6::NonSecure)
    }
}
#[doc = "Field `WrProtOfGPIO800AndGPIO804` reader - Write Protection of GPIO800 and GPIO804"]
pub type WrProtOfGpio800andGpio804R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO800AndGPIO804` writer - Write Protection of GPIO800 and GPIO804"]
pub type WrProtOfGpio800andGpio804W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Master #4 ID"]
    #[inline(always)]
    pub fn master4id(&self) -> Master4idR {
        Master4idR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Secure Master #4"]
    #[inline(always)]
    pub fn sec_master4(&self) -> SecMaster4R {
        SecMaster4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Master #5 ID"]
    #[inline(always)]
    pub fn master5id(&self) -> Master5idR {
        Master5idR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Secure Master #5"]
    #[inline(always)]
    pub fn sec_master5(&self) -> SecMaster5R {
        SecMaster5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Master #6 ID"]
    #[inline(always)]
    pub fn master6id(&self) -> Master6idR {
        Master6idR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Secure Master #6"]
    #[inline(always)]
    pub fn sec_master6(&self) -> SecMaster6R {
        SecMaster6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Protection of GPIO800 and GPIO804"]
    #[inline(always)]
    pub fn wr_prot_of_gpio800and_gpio804(&self) -> WrProtOfGpio800andGpio804R {
        WrProtOfGpio800andGpio804R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Master #4 ID"]
    #[inline(always)]
    pub fn master4id(&mut self) -> Master4idW<Gpio804Spec> {
        Master4idW::new(self, 0)
    }
    #[doc = "Bit 7 - Secure Master #4"]
    #[inline(always)]
    pub fn sec_master4(&mut self) -> SecMaster4W<Gpio804Spec> {
        SecMaster4W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Master #5 ID"]
    #[inline(always)]
    pub fn master5id(&mut self) -> Master5idW<Gpio804Spec> {
        Master5idW::new(self, 8)
    }
    #[doc = "Bit 15 - Secure Master #5"]
    #[inline(always)]
    pub fn sec_master5(&mut self) -> SecMaster5W<Gpio804Spec> {
        SecMaster5W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Master #6 ID"]
    #[inline(always)]
    pub fn master6id(&mut self) -> Master6idW<Gpio804Spec> {
        Master6idW::new(self, 16)
    }
    #[doc = "Bit 23 - Secure Master #6"]
    #[inline(always)]
    pub fn sec_master6(&mut self) -> SecMaster6W<Gpio804Spec> {
        SecMaster6W::new(self, 23)
    }
    #[doc = "Bit 31 - Write Protection of GPIO800 and GPIO804"]
    #[inline(always)]
    pub fn wr_prot_of_gpio800and_gpio804(&mut self) -> WrProtOfGpio800andGpio804W<Gpio804Spec> {
        WrProtOfGpio800andGpio804W::new(self, 31)
    }
}
#[doc = "Master Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio804::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio804::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio804Spec;
impl crate::RegisterSpec for Gpio804Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio804::R`](R) reader structure"]
impl crate::Readable for Gpio804Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio804::W`](W) writer structure"]
impl crate::Writable for Gpio804Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO804 to value 0x3f3f_3f3f"]
impl crate::Resettable for Gpio804Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
