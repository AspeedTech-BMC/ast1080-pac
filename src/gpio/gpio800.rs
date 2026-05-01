#[doc = "Register `GPIO800` reader"]
pub type R = crate::R<Gpio800Spec>;
#[doc = "Register `GPIO800` writer"]
pub type W = crate::W<Gpio800Spec>;
#[doc = "Field `Master0ID` reader - Master #0 ID"]
pub type Master0idR = crate::FieldReader;
#[doc = "Field `Master0ID` writer - Master #0 ID"]
pub type Master0idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster0 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster0> for bool {
    #[inline(always)]
    fn from(variant: SecMaster0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster0` reader - Secure Master #0"]
pub type SecMaster0R = crate::BitReader<SecMaster0>;
impl SecMaster0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster0 {
        match self.bits {
            true => SecMaster0::Secure,
            false => SecMaster0::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster0::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster0::NonSecure
    }
}
#[doc = "Field `SecMaster0` writer - Secure Master #0"]
pub type SecMaster0W<'a, REG> = crate::BitWriter<'a, REG, SecMaster0>;
impl<'a, REG> SecMaster0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster0::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster0::NonSecure)
    }
}
#[doc = "Field `Master1ID` reader - Master #1 ID"]
pub type Master1idR = crate::FieldReader;
#[doc = "Field `Master1ID` writer - Master #1 ID"]
pub type Master1idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster1 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster1> for bool {
    #[inline(always)]
    fn from(variant: SecMaster1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster1` reader - Secure Master #1"]
pub type SecMaster1R = crate::BitReader<SecMaster1>;
impl SecMaster1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster1 {
        match self.bits {
            true => SecMaster1::Secure,
            false => SecMaster1::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster1::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster1::NonSecure
    }
}
#[doc = "Field `SecMaster1` writer - Secure Master #1"]
pub type SecMaster1W<'a, REG> = crate::BitWriter<'a, REG, SecMaster1>;
impl<'a, REG> SecMaster1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster1::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster1::NonSecure)
    }
}
#[doc = "Field `Master2ID` reader - Master #2 ID"]
pub type Master2idR = crate::FieldReader;
#[doc = "Field `Master2ID` writer - Master #2 ID"]
pub type Master2idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster2 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster2> for bool {
    #[inline(always)]
    fn from(variant: SecMaster2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster2` reader - Secure Master #2"]
pub type SecMaster2R = crate::BitReader<SecMaster2>;
impl SecMaster2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster2 {
        match self.bits {
            true => SecMaster2::Secure,
            false => SecMaster2::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster2::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster2::NonSecure
    }
}
#[doc = "Field `SecMaster2` writer - Secure Master #2"]
pub type SecMaster2W<'a, REG> = crate::BitWriter<'a, REG, SecMaster2>;
impl<'a, REG> SecMaster2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster2::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster2::NonSecure)
    }
}
#[doc = "Field `Master3ID` reader - Master #3 ID"]
pub type Master3idR = crate::FieldReader;
#[doc = "Field `Master3ID` writer - Master #3 ID"]
pub type Master3idW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Secure Master #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecMaster3 {
    #[doc = "1: Secure"]
    Secure = 1,
    #[doc = "0: Non-Secure"]
    NonSecure = 0,
}
impl From<SecMaster3> for bool {
    #[inline(always)]
    fn from(variant: SecMaster3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecMaster3` reader - Secure Master #3"]
pub type SecMaster3R = crate::BitReader<SecMaster3>;
impl SecMaster3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecMaster3 {
        match self.bits {
            true => SecMaster3::Secure,
            false => SecMaster3::NonSecure,
        }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SecMaster3::Secure
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SecMaster3::NonSecure
    }
}
#[doc = "Field `SecMaster3` writer - Secure Master #3"]
pub type SecMaster3W<'a, REG> = crate::BitWriter<'a, REG, SecMaster3>;
impl<'a, REG> SecMaster3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster3::Secure)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SecMaster3::NonSecure)
    }
}
impl R {
    #[doc = "Bits 0:6 - Master #0 ID"]
    #[inline(always)]
    pub fn master0id(&self) -> Master0idR {
        Master0idR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Secure Master #0"]
    #[inline(always)]
    pub fn sec_master0(&self) -> SecMaster0R {
        SecMaster0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Master #1 ID"]
    #[inline(always)]
    pub fn master1id(&self) -> Master1idR {
        Master1idR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Secure Master #1"]
    #[inline(always)]
    pub fn sec_master1(&self) -> SecMaster1R {
        SecMaster1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Master #2 ID"]
    #[inline(always)]
    pub fn master2id(&self) -> Master2idR {
        Master2idR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Secure Master #2"]
    #[inline(always)]
    pub fn sec_master2(&self) -> SecMaster2R {
        SecMaster2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Master #3 ID"]
    #[inline(always)]
    pub fn master3id(&self) -> Master3idR {
        Master3idR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Secure Master #3"]
    #[inline(always)]
    pub fn sec_master3(&self) -> SecMaster3R {
        SecMaster3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Master #0 ID"]
    #[inline(always)]
    pub fn master0id(&mut self) -> Master0idW<Gpio800Spec> {
        Master0idW::new(self, 0)
    }
    #[doc = "Bit 7 - Secure Master #0"]
    #[inline(always)]
    pub fn sec_master0(&mut self) -> SecMaster0W<Gpio800Spec> {
        SecMaster0W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Master #1 ID"]
    #[inline(always)]
    pub fn master1id(&mut self) -> Master1idW<Gpio800Spec> {
        Master1idW::new(self, 8)
    }
    #[doc = "Bit 15 - Secure Master #1"]
    #[inline(always)]
    pub fn sec_master1(&mut self) -> SecMaster1W<Gpio800Spec> {
        SecMaster1W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Master #2 ID"]
    #[inline(always)]
    pub fn master2id(&mut self) -> Master2idW<Gpio800Spec> {
        Master2idW::new(self, 16)
    }
    #[doc = "Bit 23 - Secure Master #2"]
    #[inline(always)]
    pub fn sec_master2(&mut self) -> SecMaster2W<Gpio800Spec> {
        SecMaster2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Master #3 ID"]
    #[inline(always)]
    pub fn master3id(&mut self) -> Master3idW<Gpio800Spec> {
        Master3idW::new(self, 24)
    }
    #[doc = "Bit 31 - Secure Master #3"]
    #[inline(always)]
    pub fn sec_master3(&mut self) -> SecMaster3W<Gpio800Spec> {
        SecMaster3W::new(self, 31)
    }
}
#[doc = "Master Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio800::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio800::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio800Spec;
impl crate::RegisterSpec for Gpio800Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio800::R`](R) reader structure"]
impl crate::Readable for Gpio800Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio800::W`](W) writer structure"]
impl crate::Writable for Gpio800Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO800 to value 0x3f3f_3f3f"]
impl crate::Resettable for Gpio800Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
