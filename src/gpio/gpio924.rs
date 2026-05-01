#[doc = "Register `GPIO924` reader"]
pub type R = crate::R<Gpio924Spec>;
#[doc = "Register `GPIO924` writer"]
pub type W = crate::W<Gpio924Spec>;
#[doc = "Field `GPIO020ReadPrivilegeOfMaster` reader - GPIO020 Read Privilege of Master"]
pub type Gpio020readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO020ReadPrivilegeOfMaster` writer - GPIO020 Read Privilege of Master"]
pub type Gpio020readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO021ReadPrivilegeOfMaster` reader - GPIO021 Read Privilege of Master"]
pub type Gpio021readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO021ReadPrivilegeOfMaster` writer - GPIO021 Read Privilege of Master"]
pub type Gpio021readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO022ReadPrivilegeOfMaster` reader - GPIO022 Read Privilege of Master"]
pub type Gpio022readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO022ReadPrivilegeOfMaster` writer - GPIO022 Read Privilege of Master"]
pub type Gpio022readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO023ReadPrivilegeOfMaster` reader - GPIO023 Read Privilege of Master"]
pub type Gpio023readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO023ReadPrivilegeOfMaster` writer - GPIO023 Read Privilege of Master"]
pub type Gpio023readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO020 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio020read_privilege_of_master(&self) -> Gpio020readPrivilegeOfMasterR {
        Gpio020readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO021 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio021read_privilege_of_master(&self) -> Gpio021readPrivilegeOfMasterR {
        Gpio021readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO022 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio022read_privilege_of_master(&self) -> Gpio022readPrivilegeOfMasterR {
        Gpio022readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO023 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio023read_privilege_of_master(&self) -> Gpio023readPrivilegeOfMasterR {
        Gpio023readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO020 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio020read_privilege_of_master(
        &mut self,
    ) -> Gpio020readPrivilegeOfMasterW<Gpio924Spec> {
        Gpio020readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO021 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio021read_privilege_of_master(
        &mut self,
    ) -> Gpio021readPrivilegeOfMasterW<Gpio924Spec> {
        Gpio021readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO022 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio022read_privilege_of_master(
        &mut self,
    ) -> Gpio022readPrivilegeOfMasterW<Gpio924Spec> {
        Gpio022readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO023 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio023read_privilege_of_master(
        &mut self,
    ) -> Gpio023readPrivilegeOfMasterW<Gpio924Spec> {
        Gpio023readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio924::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio924::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio924Spec;
impl crate::RegisterSpec for Gpio924Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio924::R`](R) reader structure"]
impl crate::Readable for Gpio924Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio924::W`](W) writer structure"]
impl crate::Writable for Gpio924Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO924 to value 0xffff_ffff"]
impl crate::Resettable for Gpio924Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
