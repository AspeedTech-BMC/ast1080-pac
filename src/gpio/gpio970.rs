#[doc = "Register `GPIO970` reader"]
pub type R = crate::R<Gpio970Spec>;
#[doc = "Register `GPIO970` writer"]
pub type W = crate::W<Gpio970Spec>;
#[doc = "Field `GPIO096ReadPrivilegeOfMaster` reader - GPIO096 Read Privilege of Master"]
pub type Gpio096readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO096ReadPrivilegeOfMaster` writer - GPIO096 Read Privilege of Master"]
pub type Gpio096readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO097ReadPrivilegeOfMaster` reader - GPIO097 Read Privilege of Master"]
pub type Gpio097readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO097ReadPrivilegeOfMaster` writer - GPIO097 Read Privilege of Master"]
pub type Gpio097readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO098ReadPrivilegeOfMaster` reader - GPIO098 Read Privilege of Master"]
pub type Gpio098readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO098ReadPrivilegeOfMaster` writer - GPIO098 Read Privilege of Master"]
pub type Gpio098readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO099ReadPrivilegeOfMaster` reader - GPIO099 Read Privilege of Master"]
pub type Gpio099readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO099ReadPrivilegeOfMaster` writer - GPIO099 Read Privilege of Master"]
pub type Gpio099readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO096 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio096read_privilege_of_master(&self) -> Gpio096readPrivilegeOfMasterR {
        Gpio096readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO097 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio097read_privilege_of_master(&self) -> Gpio097readPrivilegeOfMasterR {
        Gpio097readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO098 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio098read_privilege_of_master(&self) -> Gpio098readPrivilegeOfMasterR {
        Gpio098readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO099 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio099read_privilege_of_master(&self) -> Gpio099readPrivilegeOfMasterR {
        Gpio099readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO096 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio096read_privilege_of_master(
        &mut self,
    ) -> Gpio096readPrivilegeOfMasterW<Gpio970Spec> {
        Gpio096readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO097 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio097read_privilege_of_master(
        &mut self,
    ) -> Gpio097readPrivilegeOfMasterW<Gpio970Spec> {
        Gpio097readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO098 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio098read_privilege_of_master(
        &mut self,
    ) -> Gpio098readPrivilegeOfMasterW<Gpio970Spec> {
        Gpio098readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO099 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio099read_privilege_of_master(
        &mut self,
    ) -> Gpio099readPrivilegeOfMasterW<Gpio970Spec> {
        Gpio099readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio970::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio970::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio970Spec;
impl crate::RegisterSpec for Gpio970Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio970::R`](R) reader structure"]
impl crate::Readable for Gpio970Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio970::W`](W) writer structure"]
impl crate::Writable for Gpio970Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO970 to value 0xffff_ffff"]
impl crate::Resettable for Gpio970Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
